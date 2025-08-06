import { defineStore } from 'pinia';
import { mnemonicToSeedSync, generateMnemonic, validateMnemonic } from 'bip39';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import nacl from 'tweetnacl';
import { useRouter } from 'vue-router';
import { setCurrentIdentity, clearCurrentIdentity } from '../services/canisterService.js';
import canisterService from '../services/canisterService.js';

let identity = null;

function deriveKeysFromSeedPhrase(seedPhrase) {
  const seed = mnemonicToSeedSync(seedPhrase).slice(0, 32);
  return nacl.sign.keyPair.fromSeed(seed);
}

function createIdentityFromKeyPair(keyPair) {
  return Ed25519KeyIdentity.fromKeyPair(keyPair.publicKey, keyPair.secretKey);
}

export const useAuthStore = defineStore('auth', {
  state: () => ({
    authenticated: false,
    registered: false,
    user: null,
    seedPhrase: '',
    isLoading: false,
    error: null,
  }),

  getters: {
    isAuthenticated: state => state.authenticated,
    isRegistered: state => state.registered,
    getCurrentUser: state => state.user,
    getSeedPhrase: state => state.seedPhrase,
    getPrincipal: () => (identity ? identity.getPrincipal().toText() : null),
  },

  actions: {
    // Initialize authentication on app startup
    async initializeAuth() {
      try {
        this.isLoading = true;

        // Try to load existing session from localStorage
        const loaded = await this.loadStateFromLocalStorage();

        if (loaded && this.authenticated) {
          // User has existing session, verify with backend
          await this.verifyUserRegistration();
        } else {
          // No existing session, just wait for user to log in
          console.log('No existing session found. User needs to log in.');
        }
      } catch (error) {
        console.error('Auth initialization failed:', error);
        this.error = error.message;
        // Reset state on failure
        this.$reset();
        localStorage.removeItem('authStore');
      } finally {
        this.isLoading = false;
      }
    },

    // Create a new guest account automatically
    async createGuestAccount(profile = null) {
      try {
        console.log('Creating new guest account...');

        // Generate a new 12-word seed phrase using bip39
        const seedPhrase = generateMnemonic();
        console.log('Generated seed phrase:', seedPhrase);

        // Create identity from seed phrase
        const keyPair = deriveKeysFromSeedPhrase(seedPhrase);
        identity = createIdentityFromKeyPair(keyPair);

        console.log('Identity created:', identity.getPrincipal().toText());

        // Set current identity for backend calls
        setCurrentIdentity(identity);

        // Save seed phrase and set authenticated state
        this.seedPhrase = seedPhrase;
        this.authenticated = true;
        this.registered = false;

        // Save to localStorage
        this.saveStateToLocalStorage();

        // Create user profile in backend
        try {
          console.log('Attempting to create user profile...');
          const created = await this.createUserProfile(profile);
          if (created) {
            console.log('User profile created successfully');
          } else {
            console.log('User already exists, getting existing profile...');
            await this.verifyUserRegistration();
          }
        } catch (error) {
          console.log('Error in createUserProfile:', error.message);
          // If user already exists, try to get the existing user
          if (error.message.includes('AlreadyExists')) {
            console.log('User already exists, getting existing profile...');
            await this.verifyUserRegistration();
          } else {
            throw error;
          }
        }

        console.log('Guest account created successfully');
        return { success: true, principal: identity.getPrincipal().toText() };
      } catch (error) {
        console.error('Failed to create guest account:', error);
        this.error = error.message;
        throw error;
      }
    },

    // Create user profile in the backend
    async createUserProfile(profile = null) {
      try {
        const principal = identity.getPrincipal().toText();
        
        // Use provided profile or generate default one
        const userProfile = profile || {
          name: `User_${principal.slice(0, 8)}`,
          username: (() => {
            const randomDigits = Math.floor(Math.random() * 9000) + 1000; // 1000-9999
            return `Guest${randomDigits}`;
          })(),
          email: [],
          bio: [],
          avatar_url: [],
          theme_preferences: []
        };

        // Create user profile with authenticated call
        const result = await canisterService.createUser(userProfile);

        console.log('Backend result:', result);

        if ('Ok' in result) {
          this.user = result.Ok;
          this.registered = true;
          this.saveStateToLocalStorage();
          console.log('User profile created:', this.user);
          return true;
        } else {
          // Don't throw error, just return false to indicate failure
          console.log('User profile creation failed:', result.Err);
          return false;
        }
      } catch (error) {
        console.error('Failed to create user profile:', error);
        throw error;
      }
    },

    // Verify user registration with backend
    async verifyUserRegistration() {
      try {
        if (!identity) {
          throw new Error('No identity available');
        }
        // Get user profile
        const result = await canisterService.getCurrentUser();

        if (result) {
          this.user = result;
          this.registered = true;
          this.saveStateToLocalStorage();
          console.log('User profile verified:', this.user);
          
          // Load theme preferences if available
          if (window.themeSyncService && result.profile?.theme_preferences) {
            await window.themeSyncService.loadThemeFromBackend();
          }
        } else {
          // User doesn't exist, create one
          await this.createUserProfile();
        }
      } catch (error) {
        console.error('Failed to verify user registration:', error);
        // If verification fails, reset state
        this.$reset();
        identity = null;
        localStorage.removeItem('authStore');
        throw error;
      }
    },

    // Recover account using seed phrase
    async recoverAccount(seedPhrase) {
      try {
        this.isLoading = true;

        if (!validateMnemonic(seedPhrase)) {
          throw new Error('Invalid seed phrase');
        }

        console.log('Recovering account with seed phrase...');

        // Create identity from seed phrase
        const keyPair = deriveKeysFromSeedPhrase(seedPhrase);
        identity = createIdentityFromKeyPair(keyPair);

        console.log('Identity recovered:', identity.getPrincipal().toText());

        // Set current identity for backend calls
        setCurrentIdentity(identity);

        // Set authenticated state
        this.seedPhrase = seedPhrase;
        this.authenticated = true;

        // Save to localStorage
        this.saveStateToLocalStorage();

        // Verify user registration
        await this.verifyUserRegistration();

        console.log('Account recovered successfully');
        return { success: true, principal: identity.getPrincipal().toText() };
      } catch (error) {
        console.error('Failed to recover account:', error);
        this.error = error.message;
        throw error;
      } finally {
        this.isLoading = false;
      }
    },

    // Update user profile
    async updateUserProfile(updates) {
      try {
        if (!identity) {
          throw new Error('No identity available');
        }

        const principal = identity.getPrincipal().toText();

        // Import backend canister with authenticated identity
        const { createActor } = await import('../../../declarations/backend');
        const { HttpAgent } = await import('@dfinity/agent');
        
        // Create agent with authenticated identity
        const agent = new HttpAgent({
          identity: identity,
          host: 'http://127.0.0.1:4943'
        });
        
        // Fetch root key for local development
        await agent.fetchRootKey();
        
        // Create actor with authenticated identity
        const backendActor = createActor(process.env.CANISTER_ID_BACKEND, {
          agent
        });

        // Format updates for UserProfileUpdate type
        const formattedUpdates = {
          name: updates.name,
          username: updates.username,
          email: updates.email,
          bio: updates.bio !== undefined ? [updates.bio] : [],
          avatar_url: updates.avatar_url !== undefined ? [updates.avatar_url] : [],
        };

        // Update user profile
        const result = await canisterService.updateProfile(principal, formattedUpdates);

        if ('Ok' in result) {
          this.user = result.Ok;
          this.saveStateToLocalStorage();
          console.log('User profile updated:', this.user);
          return this.user;
        } else {
          throw new Error(result.Err || 'Failed to update user profile');
        }
      } catch (error) {
        console.error('Failed to update user profile:', error);
        throw error;
      }
    },

    // Update username only
    async updateUsername(username) {
      try {
        if (!identity) {
          throw new Error('No identity available');
        }

        const principal = identity.getPrincipal().toText();

        // Update username
        const result = await canisterService.updateUsername(principal, username);

        if ('Ok' in result) {
          // Update local user state
          if (this.user) {
            this.user.profile.username = username;
            this.saveStateToLocalStorage();
          }
          console.log('Username updated:', username);
          return true;
        } else {
          throw new Error(result.Err || 'Failed to update username');
        }
      } catch (error) {
        console.error('Failed to update username:', error);
        throw error;
      }
    },

    // Logout user
    async logout() {
      try {
        // Clear all state
        this.$reset();
        identity = null;

        // Clear current identity for backend calls
        clearCurrentIdentity();

        // Clear localStorage
        localStorage.removeItem('authStore');

        console.log('User logged out successfully');

        // Redirect to home page
        const router = useRouter();
        await router.push('/');
      } catch (error) {
        console.error('Logout failed:', error);
        throw error;
      }
    },

    // Save state to localStorage
    saveStateToLocalStorage() {
      try {
        const stateToSave = {
          authenticated: this.authenticated,
          registered: this.registered,
          user: this.user,
          seedPhrase: this.seedPhrase,
        };

        // Custom JSON serializer that handles BigInt
        const jsonString = JSON.stringify(stateToSave, (key, value) => {
          if (typeof value === 'bigint') {
            return value.toString();
          }
          return value;
        });

        localStorage.setItem('authStore', jsonString);
        console.log('Auth state saved to localStorage');
      } catch (error) {
        console.error('Failed to save auth state:', error);
      }
    },

    // Load state from localStorage
    async loadStateFromLocalStorage() {
      try {
        const stored = localStorage.getItem('authStore');
        if (!stored) {
          return false;
        }

        const parsed = JSON.parse(stored);

        // Restore state
        this.authenticated = parsed.authenticated || false;
        this.registered = parsed.registered || false;
        this.user = parsed.user || null;
        this.seedPhrase = parsed.seedPhrase || '';

        // Recreate identity if seed phrase exists
        if (this.seedPhrase && this.authenticated) {
          const keyPair = deriveKeysFromSeedPhrase(this.seedPhrase);
          identity = createIdentityFromKeyPair(keyPair);
          console.log(
            'Identity restored from localStorage:',
            identity.getPrincipal().toText()
          );
          
          // Set current identity for backend calls
          setCurrentIdentity(identity);
        }

        console.log('Auth state loaded from localStorage');
        return true;
      } catch (error) {
        console.error('Failed to load auth state:', error);
        // Clear invalid state
        localStorage.removeItem('authStore');
        return false;
      }
    },

    // Get current identity for canister calls
    getIdentity() {
      return identity;
    },

    // Login with profile (for manual registration)
    async login(profile) {
      try {
        this.isLoading = true;
        this.clearError();

        // Generate a new seed phrase for this user
        const seedPhrase = generateMnemonic();
        
        // Create identity from seed phrase
        const keyPair = deriveKeysFromSeedPhrase(seedPhrase);
        identity = createIdentityFromKeyPair(keyPair);

        // Set current identity for backend calls
        setCurrentIdentity(identity);

        // Set authenticated state
        this.seedPhrase = seedPhrase;
        this.authenticated = true;
        this.registered = false;

        // Save to localStorage
        this.saveStateToLocalStorage();

        // Create user profile with provided data
        const result = await canisterService.createUser({
          name: profile.name,
          username: profile.username || (() => {
            const randomDigits = Math.floor(Math.random() * 9000) + 1000; // 1000-9999
            return `Guest${randomDigits}`;
          })(),
          email: profile.email ? [profile.email] : [],
          bio: profile.bio ? [profile.bio] : [],
          avatar_url: profile.avatar_url ? [profile.avatar_url] : [],
          theme_preferences: []
        });

        if ('Ok' in result) {
          // Get the created user
          const userResult = await canisterService.getUser(result.Ok);
          if ('Ok' in userResult && userResult.Ok) {
            this.user = userResult.Ok;
            this.registered = true;
            this.saveStateToLocalStorage();
            console.log('User profile created:', this.user);
            return { success: true, principal: identity.getPrincipal().toText() };
          }
        }
        
        throw new Error(result.Err || 'Failed to create user profile');
      } catch (error) {
        console.error('Login failed:', error);
        this.error = error.message;
        // Reset state on failure
        this.$reset();
        identity = null;
        localStorage.removeItem('authStore');
        throw error;
      } finally {
        this.isLoading = false;
      }
    },

    // Clear error state
    clearError() {
      this.error = null;
    },
  },
});
