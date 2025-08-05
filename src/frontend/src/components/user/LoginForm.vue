<template>
  <div class="min-h-screen flex items-center justify-center bg-background p-4">
    <Card class="w-full max-w-md relative">
      <!-- Theme Switchers - Top Right -->
      <div v-if="currentStep === 1" class="absolute top-4 right-4 flex space-x-2 z-10">
        <!-- Color Switcher -->
        <Button
          variant="outline"
          size="icon"
          class="w-8 h-8 relative overflow-hidden"
          :style="{ backgroundColor: `hsl(${currentColorConfig.primary})` }"
          @click="cycleColor"
        >
          <div class="w-4 h-4 rounded-full border-2 border-white/40 bg-white/30 flex items-center justify-center">
            <div class="w-2 h-2 rounded-full shadow-sm" :style="{ backgroundColor: `hsl(${currentColorConfig.primary})` }"></div>
          </div>
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

      <CardHeader class="text-center">
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
          <!-- Progress Indicator -->
          <div class="space-y-2">
            <div class="flex justify-between text-sm text-muted-foreground">
              <span>Step 2 of 3</span>
              <span>Profile Setup</span>
            </div>
            <div class="w-full bg-muted rounded-full h-2">
              <div class="bg-primary h-2 rounded-full transition-all duration-300" style="width: 66%"></div>
            </div>
          </div>

          <div class="text-center">
            <h2 class="text-xl font-semibold">Tell us about yourself</h2>
            <p class="text-sm text-muted-foreground mt-2">
              This helps personalize your experience
            </p>
          </div>
          
          <div class="space-y-4">
            <!-- Name Field -->
            <div>
              <Label for="name" class="flex items-center gap-2">
                Full Name
                <span class="text-destructive">*</span>
                <div v-if="nameValidation.isValid && profile.name" class="flex-shrink-0">
                  <svg class="w-4 h-4 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                  </svg>
                </div>
              </Label>
              <Input
                id="name"
                v-model="profile.name"
                placeholder="Enter your full name"
                class="mt-1"
                :class="nameValidation.isValid && profile.name ? 'border-green-500' : nameValidation.error ? 'border-destructive' : ''"
                @input="validateName"
              />
              <p v-if="nameValidation.error" class="text-sm text-destructive mt-1">{{ nameValidation.error }}</p>
            </div>
            
            <!-- Email Field -->
            <div>
              <Label for="email" class="flex items-center gap-2">
                Email Address
                <span class="text-destructive">*</span>
                <div v-if="emailValidation.isValid && profile.email" class="flex-shrink-0">
                  <svg class="w-4 h-4 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                  </svg>
                </div>
              </Label>
              <Input
                id="email"
                v-model="profile.email"
                type="email"
                placeholder="Enter your email"
                class="mt-1"
                :class="emailValidation.isValid && profile.email ? 'border-green-500' : emailValidation.error ? 'border-destructive' : ''"
                @input="validateEmail"
              />
              <p v-if="emailValidation.error" class="text-sm text-destructive mt-1">{{ emailValidation.error }}</p>
            </div>
            
            <!-- Bio Field (Collapsible) -->
            <div>
              <Button
                variant="ghost"
                class="p-0 h-auto text-sm text-muted-foreground hover:text-foreground"
                @click="showBio = !showBio"
              >
                <div class="flex items-center gap-2">
                  <svg 
                    class="w-4 h-4 transition-transform" 
                    :class="showBio ? 'rotate-90' : ''"
                    fill="currentColor" 
                    viewBox="0 0 20 20"
                  >
                    <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
                  </svg>
                  Add bio (optional)
                </div>
              </Button>
              
              <div v-if="showBio" class="mt-3 space-y-2">
                <Label for="bio">Bio</Label>
                <Textarea
                  id="bio"
                  v-model="profile.bio"
                  placeholder="Tell us about yourself"
                  rows="3"
                  maxlength="200"
                />
                <p class="text-xs text-muted-foreground text-right">
                  {{ profile.bio.length }}/200 characters
                </p>
              </div>
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
            <h2 class="text-xl font-semibold">Account Created!</h2>
            <p class="text-sm text-muted-foreground mt-2">
              Your account has been successfully created
            </p>
          </div>
          
          <div class="bg-amber-50 border border-amber-200 rounded-lg p-4">
            <div class="flex items-start gap-3">
              <svg class="w-5 h-5 text-amber-600 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
              </svg>
              <div>
                <h4 class="font-medium text-amber-800">Save Your Recovery Phrase</h4>
                <p class="text-sm text-amber-700 mt-1">
                  Write down this 12-word phrase to recover your account later:
                </p>
                <div class="mt-3 p-3 bg-white border border-amber-300 rounded font-mono text-sm">
                  {{ seedPhrase }}
                </div>
              </div>
            </div>
          </div>
          
          <Button 
            class="w-full"
            @click="goToDashboard"
          >
            Continue to Dashboard
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
import { ref, reactive, computed } from 'vue';
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
const showBio = ref(false);

const profile = reactive({
  name: '',
  email: '',
  bio: ''
});

// Validation states
const nameValidation = reactive({
  isValid: false,
  error: ''
});

const emailValidation = reactive({
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
  return nameValidation.isValid && emailValidation.isValid && profile.name && profile.email;
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
const validateName = () => {
  const name = profile.name.trim();
  if (!name) {
    nameValidation.isValid = false;
    nameValidation.error = '';
    return;
  }
  
  if (name.length < 2) {
    nameValidation.isValid = false;
    nameValidation.error = 'Name must be at least 2 characters long';
    return;
  }
  
  if (!/^[a-zA-Z\s]+$/.test(name)) {
    nameValidation.isValid = false;
    nameValidation.error = 'Name can only contain letters and spaces';
    return;
  }
  
  nameValidation.isValid = true;
  nameValidation.error = '';
};

const validateEmail = () => {
  const email = profile.email.trim();
  if (!email) {
    emailValidation.isValid = false;
    emailValidation.error = '';
    return;
  }
  
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  if (!emailRegex.test(email)) {
    emailValidation.isValid = false;
    emailValidation.error = 'Please enter a valid email address';
    return;
  }
  
  emailValidation.isValid = true;
  emailValidation.error = '';
};

const startOnboarding = () => {
  currentStep.value = 2;
};

const createAccount = async () => {
  try {
    loading.value = true;
    
    // Get current theme preferences
    const themePreferences = {
      theme: themeStore.currentTheme,
      color: themeStore.currentColor,
      dark_mode: isDark.value
    };
    
    // Create the user profile
    const userProfile = {
      name: profile.name,
      email: profile.email,
      bio: profile.bio || null,
      avatar_url: null,
      theme_preferences: themePreferences
    };
    
    // Create guest account with profile
    await authStore.createGuestAccount(userProfile);
    
    // Get the seed phrase for display
    seedPhrase.value = authStore.getSeedPhrase;
    
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
    
    await authStore.recoverAccount(signInSeedPhrase.value.trim());
    
    // Close modal and redirect
    showSignInModal.value = false;
    signInSeedPhrase.value = '';
    
    // Redirect to dashboard
    await router.push('/');
  } catch (error) {
    console.error('Sign in failed:', error);
    signInError.value = error.message || 'Failed to sign in. Please check your recovery phrase.';
  } finally {
    signInLoading.value = false;
  }
};
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 1rem;
}

.login-panel {
  background: white;
  border-radius: 16px;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  padding: 3rem;
  max-width: 400px;
  width: 100%;
}

.logo-section {
  text-align: center;
  margin-bottom: 2rem;
}

.app-title {
  font-size: 2rem;
  font-weight: 700;
  color: #1f2937;
  margin-bottom: 0.5rem;
}

.app-subtitle {
  color: #6b7280;
  font-size: 1rem;
}

.auth-section {
  text-align: center;
}

.section-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 1.5rem;
}

.auth-button {
  width: 100%;
  padding: 0.75rem 1rem;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  margin-bottom: 1rem;
}

.guest-button {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.guest-button:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
}

.guest-button:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

.recovery-button {
  background: #f3f4f6;
  color: #374151;
  border: 1px solid #d1d5db;
}

.recovery-button:hover {
  background: #e5e7eb;
}

.button-content {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}

.button-icon {
  width: 1.25rem;
  height: 1.25rem;
}

.button-text {
  font-weight: 500;
}

.divider {
  position: relative;
  margin: 1.5rem 0;
}

.divider::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 0;
  right: 0;
  height: 1px;
  background: #e5e7eb;
}

.divider-text {
  background: white;
  padding: 0 1rem;
  color: #6b7280;
  font-size: 0.875rem;
}

.info-text {
  margin-top: 1.5rem;
  text-align: left;
}

.info-text p {
  color: #6b7280;
  font-size: 0.875rem;
  margin-bottom: 0.5rem;
  line-height: 1.4;
}

/* Modal Styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 1rem;
}

.modal-content {
  background: white;
  border-radius: 12px;
  max-width: 500px;
  width: 100%;
  max-height: 90vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #e5e7eb;
}

.modal-header h3 {
  font-size: 1.25rem;
  font-weight: 600;
  color: #1f2937;
  margin: 0;
}

.close-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 4px;
  color: #6b7280;
}

.close-button:hover {
  background: #f3f4f6;
}

.modal-body {
  padding: 1.5rem;
}

.modal-description {
  color: #6b7280;
  margin-bottom: 1rem;
  line-height: 1.5;
}

.seed-phrase-input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-family: monospace;
  font-size: 0.875rem;
  resize: vertical;
  margin-bottom: 1rem;
}

.seed-phrase-input:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.error-message {
  color: #dc2626;
  font-size: 0.875rem;
  margin-bottom: 1rem;
  padding: 0.5rem;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 4px;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
}

.recovery-submit-button {
  background: #667eea;
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.recovery-submit-button:hover:not(:disabled) {
  background: #5a67d8;
}

.recovery-submit-button:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

/* Responsive Design */
@media (max-width: 640px) {
  .login-panel {
    padding: 2rem;
    margin: 1rem;
  }
  
  .app-title {
    font-size: 1.75rem;
  }
  
  .modal-content {
    margin: 1rem;
  }
}
</style> 