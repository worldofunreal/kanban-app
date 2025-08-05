<template>
  <div class="min-h-screen flex items-center justify-center bg-background p-4">
    <Card class="w-full max-w-md">
      <CardHeader class="text-center">
        <CardTitle class="text-2xl font-bold">Kanban App</CardTitle>
        <CardDescription>Collaborative Project Management</CardDescription>
      </CardHeader>
      
      <CardContent class="space-y-6">
        <div class="text-center">
          <h2 class="text-xl font-semibold">Get Started</h2>
        </div>
        
        <!-- Guest Account Button -->
        <Button 
          :disabled="loading" 
          class="w-full"
          @click="handleGuestLogin"
        >
          <div class="flex items-center justify-center gap-2">
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd" />
            </svg>
            <span>
              {{ loading ? 'Creating Account...' : 'Continue as Guest' }}
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

        <!-- Account Recovery -->
        <Button 
          variant="outline"
          class="w-full"
          @click="showRecoveryModal = true"
        >
          <div class="flex items-center justify-center gap-2">
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd" />
            </svg>
            <span>Recover Account</span>
          </div>
        </Button>

        <div class="text-center text-sm text-muted-foreground space-y-1">
          <p>Guest accounts are automatically created and saved locally.</p>
          <p>Save your seed phrase to recover your account later.</p>
        </div>
      </CardContent>
    </Card>

    <!-- Recovery Modal -->
    <div v-if="showRecoveryModal" class="fixed inset-0 bg-background/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <Card class="w-full max-w-md">
        <CardHeader>
          <div class="flex items-center justify-between">
            <CardTitle>Recover Account</CardTitle>
            <Button variant="ghost" size="icon" @click="showRecoveryModal = false">
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
            </Button>
          </div>
        </CardHeader>
        
        <CardContent class="space-y-4">
          <p class="text-sm text-muted-foreground">
            Enter your 12-word seed phrase to recover your account:
          </p>
          
          <textarea
            v-model="recoverySeedPhrase"
            placeholder="Enter your 12-word seed phrase..."
            class="w-full min-h-[80px] p-3 border border-input bg-background rounded-md text-sm font-mono resize-none focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
          ></textarea>
          
          <div v-if="recoveryError" class="p-3 bg-destructive/10 border border-destructive/20 rounded-md">
            <p class="text-sm text-destructive">{{ recoveryError }}</p>
          </div>
          
          <div class="flex justify-end">
            <Button 
              :disabled="recoveryLoading || !recoverySeedPhrase.trim()"
              @click="handleRecovery"
            >
              {{ recoveryLoading ? 'Recovering...' : 'Recover Account' }}
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { useRouter } from 'vue-router';
import { Button } from '@/components/ui/Button.vue';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/Card.vue';

const authStore = useAuthStore();
const router = useRouter();

const loading = ref(false);
const showRecoveryModal = ref(false);
const recoverySeedPhrase = ref('');
const recoveryLoading = ref(false);
const recoveryError = ref('');

const handleGuestLogin = async () => {
  try {
    loading.value = true;
    await authStore.createGuestAccount();
    
    // Show seed phrase to user
    showSeedPhraseModal();
  } catch (error) {
    console.error('Guest login failed:', error);
    alert('Failed to create guest account. Please try again.');
  } finally {
    loading.value = false;
  }
};

const handleRecovery = async () => {
  try {
    recoveryLoading.value = true;
    recoveryError.value = '';
    
    await authStore.recoverAccount(recoverySeedPhrase.value.trim());
    
    // Close modal and redirect
    showRecoveryModal.value = false;
    recoverySeedPhrase.value = '';
    
    // Redirect to dashboard
    await router.push('/');
  } catch (error) {
    console.error('Recovery failed:', error);
    recoveryError.value = error.message || 'Failed to recover account';
  } finally {
    recoveryLoading.value = false;
  }
};

const showSeedPhraseModal = () => {
  const seedPhrase = authStore.getSeedPhrase;
  const message = `Your account has been created successfully!\n\nIMPORTANT: Save this seed phrase to recover your account:\n\n${seedPhrase}\n\nWrite it down and keep it safe!`;
  
  alert(message);
  
  // Redirect to dashboard
  router.push('/');
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