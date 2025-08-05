<template>
  <div class="min-h-screen flex items-center justify-center bg-background p-4">
    <Card class="w-full max-w-md relative">
      <!-- Theme Switchers - Top Right -->
      <div v-if="currentStep === 1" class="absolute top-4 right-4 flex space-x-2 z-10">
        <!-- Color Switcher -->
        <Button
          variant="outline"
          size="icon"
          class="w-8 h-8"
          @click="cycleColor"
        >
          <div class="w-4 h-4 rounded-full bg-primary"></div>
        </Button>
        
        <!-- Theme Switcher -->
        <Button
          variant="outline"
          size="icon"
          class="w-8 h-8"
          @click="themeStore.toggleDarkMode"
        >
          <svg
            v-if="!isDark"
            class="h-4 w-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
          </svg>
          <svg
            v-else
            class="h-4 w-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
          </svg>
        </Button>
      </div>

      <CardHeader class="text-center" v-if="currentStep === 1">
        <CardTitle class="text-2xl font-bold">Kanban App</CardTitle>
        <CardDescription>Collaborative Project Management</CardDescription>
      </CardHeader>
      
      <CardContent class="space-y-6">
        <!-- Step 1: Welcome -->
        <div v-if="currentStep === 1" class="space-y-6">
          <div class="text-center">
            <h2 class="text-xl font-semibold">Welcome!</h2>
            <p class="text-sm text-muted-foreground mt-2">
              Get started with your collaborative workspace
            </p>
          </div>
          
          <Button 
            :disabled="loading" 
            class="w-full"
            @click="startOnboarding"
          >
            <div class="flex items-center justify-center gap-2">
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-8.707l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 001.414 1.414L9 9.414V13a1 1 0 102 0V9.414l1.293 1.293a1 1 0 001.414-1.414z" clip-rule="evenodd" />
              </svg>
              <span>
                {{ loading ? 'Setting up...' : 'Get Started' }}
              </span>
            </div>
          </Button>

          <div class="relative">
            <div class="absolute inset-0 flex items-center">
              <span class="w-full border-t" />
            </div>
            <div class="relative flex justify-center text-xs uppercase">
              <span class="bg-background px-2 text-muted-foreground">or</span>
            </div>
          </div>

          <Button 
            variant="outline"
            class="w-full"
            @click="showSignInModal = true"
          >
            <div class="flex items-center justify-center gap-2">
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M3 3a1 1 0 011-1h12a1 1 0 011 1v12a1 1 0 01-1 1H4a1 1 0 01-1-1V3zm10.293 9.293a1 1 0 001.414 1.414l3-3a1 1 0 000-1.414l-3-3a1 1 0 10-1.414 1.414L14.586 9H7a1 1 0 100 2h7.586l-1.293 1.293z" clip-rule="evenodd" />
              </svg>
              <span>Sign In</span>
            </div>
          </Button>
        </div>

        <!-- Step 2: Profile Setup -->
        <div v-if="currentStep === 2" class="space-y-6">
          <div class="text-center mt-4">
            <h2 class="text-xl font-semibold">Customize your profile</h2>
            <p class="text-sm text-muted-foreground mt-2">
              This is how other users will see you in the workspace
            </p>
          </div>
          
          <div class="space-y-4">
            <!-- Username Field -->
            <div>
              <Label for="username" class="flex mb-2 items-center gap-2">
                Username
                <div v-if="usernameValidation.isValid && profile.username" class="flex-shrink-0">
                  <svg class="w-4 h-4 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                  </svg>
                </div>
              </Label>
              <Input
                id="username"
                v-model="profile.username"
                placeholder="Enter your username"
                class="mt-1"
                :class="usernameValidation.isValid && profile.username ? 'border-green-500' : usernameValidation.error ? 'border-destructive' : ''"
                @input="validateUsername"
                @blur="validateUsername"
              />
              <p v-if="usernameValidation.error" class="text-sm text-destructive mt-1">{{ usernameValidation.error }}</p>
            </div>
          </div>
          
          <div class="flex gap-3">
            <Button 
              variant="outline" 
              class="flex-1"
              @click="currentStep = 1"
            >
              Back
            </Button>
            <Button 
              :disabled="!isStep2Valid || loading"
              class="flex-1"
              @click="createAccount"
            >
              {{ loading ? 'Creating Account...' : 'Continue' }}
            </Button>
          </div>
        </div>

        <!-- Step 3: Success -->
        <div v-if="currentStep === 3" class="space-y-6">
          <div class="text-center">
            <div class="mx-auto w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mb-4">
              <svg class="w-8 h-8 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
              </svg>
            </div>
            <h2 class="text-xl font-semibold">Welcome to Kanban!</h2>
            <p class="text-sm text-muted-foreground mt-2">
              Your account is ready. Let's get you started with your first project.
            </p>
          </div>
          
          <Button 
            class="w-full"
            @click="goToDashboard"
          >
            Get Started
          </Button>
        </div>
      </CardContent>
    </Card>

    <!-- Sign In Modal -->
    <div v-if="showSignInModal" class="fixed inset-0 bg-background/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <Card class="w-full max-w-md">
        <CardHeader>
          <div class="flex items-center justify-between">
            <CardTitle>Sign In</CardTitle>
            <Button variant="ghost" size="icon" @click="showSignInModal = false">
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
            </Button>
          </div>
        </CardHeader>
        
        <CardContent class="space-y-4">
          <p class="text-sm text-muted-foreground">
            Enter your 12-word recovery phrase to sign in:
          </p>
          
          <Textarea
            v-model="signInSeedPhrase"
            placeholder="Enter your 12-word recovery phrase..."
            rows="4"
            class="font-mono"
          />
          
          <div v-if="signInError" class="p-3 bg-destructive/10 border border-destructive/20 rounded-md">
            <p class="text-sm text-destructive">{{ signInError }}</p>
          </div>
          
          <div class="flex justify-end">
            <Button 
              :disabled="signInLoading || !signInSeedPhrase.trim()"
              @click="handleSignIn"
            >
              {{ signInLoading ? 'Signing In...' : 'Sign In' }}
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, watch } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { useThemeStore } from '@/stores/theme';
import { useRouter } from 'vue-router';
import Button from '@/components/ui/Button.vue';
import Card from '@/components/ui/Card.vue';
import CardHeader from '@/components/ui/CardHeader.vue';
import CardTitle from '@/components/ui/CardTitle.vue';
import CardDescription from '@/components/ui/CardDescription.vue';
import CardContent from '@/components/ui/CardContent.vue';
import Input from '@/components/ui/Input.vue';
import Label from '@/components/ui/Label.vue';
import Textarea from '@/components/ui/Textarea.vue';

const authStore = useAuthStore();
const themeStore = useThemeStore();
const router = useRouter();

const currentStep = ref(1);
const loading = ref(false);
const showSignInModal = ref(false);
const signInSeedPhrase = ref('');
const signInLoading = ref(false);
const signInError = ref('');
const seedPhrase = ref('');

const profile = reactive({
  username: ''
});

// Validation states
const usernameValidation = reactive({
  isValid: false,
  error: ''
});





const currentColorConfig = computed(() => {
  return themeStore.colors[themeStore.currentColor] || themeStore.colors.neutral;
});

const isDark = computed(() => {
  return document.documentElement.classList.contains('dark');
});

const isStep2Valid = computed(() => {
  // Username is optional, so step is always valid
  return true;
});

// Color cycling
const colorKeys = Object.keys(themeStore.colors);
let currentColorIndex = colorKeys.indexOf(themeStore.currentColor);

const cycleColor = () => {
  currentColorIndex = (currentColorIndex + 1) % colorKeys.length;
  const nextColor = colorKeys[currentColorIndex];
  themeStore.setColor(nextColor);
};

// Validation functions
const validateUsername = () => {
  const username = profile.username;
  if (!username || !username.trim()) {
    usernameValidation.isValid = false;
    usernameValidation.error = '';
    return;
  }
  
  const trimmedUsername = username.trim();
  
  if (trimmedUsername.length < 2) {
    usernameValidation.isValid = false;
    usernameValidation.error = 'Username must be at least 2 characters long';
    return;
  }
  
  if (trimmedUsername.length > 12) {
    usernameValidation.isValid = false;
    usernameValidation.error = 'Username must be less than 12 characters';
    return;
  }
  
  if (!/^[\p{L}\p{N}_]+$/u.test(trimmedUsername)) {
    usernameValidation.isValid = false;
    usernameValidation.error = 'Username can only contain letters, numbers, and underscores';
    return;
  }
  
  usernameValidation.isValid = true;
  usernameValidation.error = '';
};

const startOnboarding = () => {
  // Just move to next step - let user type their own username
  currentStep.value = 2;
};

const createAccount = async () => {
  try {
    loading.value = true;
    
    // Get current theme preferences - extract actual values from refs
    const themePreferences = {
      color: themeStore.currentColor?.value || 'emerald',
      dark_mode: isDark.value
    };
    
    // Create the user profile - match exact Candid field order
    const userProfile = {
      bio: [], // Bio is optional - empty array for opt text
      username: profile.username || 'User', // Use the actual username from form, fallback to 'User'
      avatar_url: [], // Avatar is optional - empty array for opt text
      name: profile.username || 'User', // Name is required text, use username or fallback
      theme_preferences: [themePreferences], // Theme preferences - convert to opt record format
      email: [] // Email is optional - empty array for opt text
    };
    
    // Create guest account with profile
    await authStore.createGuestAccount(userProfile);
    
    // Move to success step
    currentStep.value = 3;
  } catch (error) {
    console.error('Account creation failed:', error);
    alert('Failed to create account. Please try again.');
  } finally {
    loading.value = false;
  }
};

const goToDashboard = async () => {
  await router.push('/');
};

const handleSignIn = async () => {
  try {
    signInLoading.value = true;
    signInError.value = '';
    
    console.log('Attempting to sign in with seed phrase...');
    
    // Recover account using seed phrase
    await authStore.recoverAccount(signInSeedPhrase.value.trim());
    
    // Check if user exists in backend
    console.log('Checking if user exists in backend...');
    const { backend } = await import('../../../../declarations/backend');
    const isRegistered = await backend.is_user_registered();
    
    console.log('User registration status:', isRegistered);
    
    if (isRegistered) {
      console.log('User exists, proceeding to dashboard...');
      // Close modal and redirect to dashboard
      showSignInModal.value = false;
      signInSeedPhrase.value = '';
      await router.push('/');
    } else {
      console.log('User does not exist, redirecting to registration...');
      // User doesn't exist, redirect to registration
      showSignInModal.value = false;
      signInSeedPhrase.value = '';
      // Stay on LoginForm for registration
      currentStep.value = 2;
    }
  } catch (error) {
    console.error('Sign in failed:', error);
    signInError.value = error.message || 'Failed to sign in. Please check your recovery phrase.';
  } finally {
    signInLoading.value = false;
  }
};
</script>

<style scoped>
/* Add any additional styles here */
</style>
