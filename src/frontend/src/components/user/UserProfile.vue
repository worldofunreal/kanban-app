<template>
  <div class="profile-container">
    <div class="profile-card">
      <div class="profile-header">
        <h2 class="profile-title">Account Settings</h2>
        <p class="profile-subtitle">Manage your account and preferences</p>
      </div>

      <!-- User Information -->
      <div class="profile-section">
        <h3 class="section-title">Account Information</h3>
        
        <div class="info-grid">
          <div class="info-item">
            <label class="info-label">Username</label>
            <div class="info-value">{{ user?.username || 'Not set' }}</div>
          </div>
          
          <div class="info-item">
            <label class="info-label">Principal ID</label>
            <div class="info-value principal-id">
              {{ principal }}
              <button class="copy-button" title="Copy Principal" @click="copyPrincipal">
                <svg fill="currentColor" viewBox="0 0 20 20">
                  <path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" />
                  <path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z" />
                </svg>
              </button>
            </div>
          </div>
          
          <div class="info-item">
            <label class="info-label">Email</label>
            <div class="info-value">{{ user?.email || 'Not set' }}</div>
          </div>
          
          <div class="info-item">
            <label class="info-label">Bio</label>
            <div class="info-value">{{ user?.bio || 'No bio added' }}</div>
          </div>
        </div>
      </div>

      <!-- Seed Phrase Recovery -->
      <div class="profile-section">
        <h3 class="section-title">Account Recovery</h3>
        
        <div class="recovery-info">
          <div class="warning-box">
            <svg class="warning-icon" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
            </svg>
            <div class="warning-content">
              <h4>Save Your Seed Phrase</h4>
              <p>This is the only way to recover your account. Write it down and keep it safe!</p>
            </div>
          </div>
          
          <div class="seed-phrase-section">
            <label class="seed-label">Your Seed Phrase (12 words)</label>
            <div class="seed-phrase-display">
              <span v-if="showSeedPhrase" class="seed-phrase">{{ seedPhrase }}</span>
              <span v-else class="seed-phrase-hidden">•••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• ••••••••••••</span>
            </div>
            
            <div class="seed-actions">
              <button 
                class="toggle-button" 
                :class="{ 'danger': showSeedPhrase }"
                @click="toggleSeedPhrase"
              >
                {{ showSeedPhrase ? 'Hide Seed Phrase' : 'Show Seed Phrase' }}
              </button>
              
              <button 
                v-if="showSeedPhrase"
                class="copy-seed-button" 
                @click="copySeedPhrase"
              >
                Copy to Clipboard
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Account Recovery -->
      <div class="profile-section">
        <h3 class="section-title">Recover Different Account</h3>
        
        <div class="recovery-form">
          <p class="form-description">
            Enter a different seed phrase to switch to another account:
          </p>
          
          <textarea
            v-model="recoverySeedPhrase"
            placeholder="Enter 12-word seed phrase..."
            class="recovery-input"
            rows="3"
          ></textarea>
          
          <div v-if="recoveryError" class="error-message">
            {{ recoveryError }}
          </div>
          
          <button 
            :disabled="recoveryLoading || !recoverySeedPhrase.trim()"
            class="recovery-button"
            @click="handleRecovery"
          >
            {{ recoveryLoading ? 'Recovering...' : 'Recover Account' }}
          </button>
        </div>
      </div>

      <!-- Logout -->
      <div class="profile-section">
        <h3 class="section-title">Account Actions</h3>
        
        <div class="action-buttons">
          <button class="logout-button" @click="handleLogout">
            <svg class="button-icon" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M3 3a1 1 0 00-1 1v12a1 1 0 102 0V4a1 1 0 00-1-1zm10.293 9.293a1 1 0 001.414 1.414l3-3a1 1 0 000-1.414l-3-3a1 1 0 10-1.414 1.414L14.586 9H7a1 1 0 100 2h7.586l-1.293 1.293z" clip-rule="evenodd" />
            </svg>
            Logout
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { useAuthStore } from '@/stores/auth';

const authStore = useAuthStore();

const showSeedPhrase = ref(false);
const recoverySeedPhrase = ref('');
const recoveryLoading = ref(false);
const recoveryError = ref('');

const user = computed(() => authStore.getCurrentUser);
const principal = computed(() => authStore.getPrincipal);
const seedPhrase = computed(() => authStore.getSeedPhrase);

const toggleSeedPhrase = () => {
  showSeedPhrase.value = !showSeedPhrase.value;
};

const copySeedPhrase = async () => {
  try {
    await navigator.clipboard.writeText(seedPhrase.value);
    alert('Seed phrase copied to clipboard!');
  } catch (error) {
    console.error('Failed to copy seed phrase:', error);
    alert('Failed to copy seed phrase. Please copy it manually.');
  }
};

const copyPrincipal = async () => {
  try {
    await navigator.clipboard.writeText(principal.value);
    alert('Principal ID copied to clipboard!');
  } catch (error) {
    console.error('Failed to copy principal:', error);
    alert('Failed to copy principal ID. Please copy it manually.');
  }
};

const handleRecovery = async () => {
  try {
    recoveryLoading.value = true;
    recoveryError.value = '';
    
    await authStore.recoverAccount(recoverySeedPhrase.value.trim());
    
    // Clear form
    recoverySeedPhrase.value = '';
    
    // Show success message
    alert('Account recovered successfully!');
    
    // Refresh page to update user data
    window.location.reload();
  } catch (error) {
    console.error('Recovery failed:', error);
    recoveryError.value = error.message || 'Failed to recover account';
  } finally {
    recoveryLoading.value = false;
  }
};

const handleLogout = async () => {
  if (confirm('Are you sure you want to logout? You will need your seed phrase to recover your account.')) {
    try {
      await authStore.logout();
    } catch (error) {
      console.error('Logout failed:', error);
      alert('Logout failed. Please try again.');
    }
  }
};
</script>

<style scoped>
.profile-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
}

.profile-card {
  background: white;
  border-radius: 12px;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.profile-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 2rem;
  text-align: center;
}

.profile-title {
  font-size: 1.875rem;
  font-weight: 700;
  margin-bottom: 0.5rem;
}

.profile-subtitle {
  opacity: 0.9;
  font-size: 1rem;
}

.profile-section {
  padding: 2rem;
  border-bottom: 1px solid #e5e7eb;
}

.profile-section:last-child {
  border-bottom: none;
}

.section-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 1.5rem;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
}

.info-item {
  display: flex;
  flex-direction: column;
}

.info-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: #6b7280;
  margin-bottom: 0.5rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.info-value {
  font-size: 1rem;
  color: #1f2937;
  font-weight: 500;
}

.principal-id {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-family: monospace;
  font-size: 0.875rem;
  word-break: break-all;
}

.copy-button {
  background: none;
  border: none;
  color: #6b7280;
  cursor: pointer;
  padding: 0.25rem;
  border-radius: 4px;
  transition: color 0.2s;
}

.copy-button:hover {
  color: #374151;
}

.recovery-info {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.warning-box {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  padding: 1rem;
  background: #fef3c7;
  border: 1px solid #f59e0b;
  border-radius: 8px;
}

.warning-icon {
  width: 1.5rem;
  height: 1.5rem;
  color: #d97706;
  flex-shrink: 0;
  margin-top: 0.125rem;
}

.warning-content h4 {
  font-size: 1rem;
  font-weight: 600;
  color: #92400e;
  margin-bottom: 0.25rem;
}

.warning-content p {
  font-size: 0.875rem;
  color: #92400e;
  line-height: 1.5;
}

.seed-phrase-section {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.seed-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: #374151;
}

.seed-phrase-display {
  padding: 1rem;
  background: #f9fafb;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-family: monospace;
  font-size: 0.875rem;
  line-height: 1.6;
  word-spacing: 0.5rem;
}

.seed-phrase {
  color: #1f2937;
}

.seed-phrase-hidden {
  color: #6b7280;
  letter-spacing: 0.25rem;
}

.seed-actions {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.toggle-button {
  padding: 0.5rem 1rem;
  border: 1px solid #d1d5db;
  background: white;
  color: #374151;
  border-radius: 6px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.toggle-button:hover {
  background: #f9fafb;
}

.toggle-button.danger {
  border-color: #dc2626;
  color: #dc2626;
}

.toggle-button.danger:hover {
  background: #fef2f2;
}

.copy-seed-button {
  padding: 0.5rem 1rem;
  border: 1px solid #667eea;
  background: #667eea;
  color: white;
  border-radius: 6px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.copy-seed-button:hover {
  background: #5a67d8;
}

.recovery-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-description {
  color: #6b7280;
  font-size: 0.875rem;
  line-height: 1.5;
}

.recovery-input {
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-family: monospace;
  font-size: 0.875rem;
  resize: vertical;
}

.recovery-input:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.error-message {
  color: #dc2626;
  font-size: 0.875rem;
  padding: 0.5rem;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 4px;
}

.recovery-button {
  align-self: flex-start;
  padding: 0.75rem 1.5rem;
  border: 1px solid #667eea;
  background: #667eea;
  color: white;
  border-radius: 6px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.recovery-button:hover:not(:disabled) {
  background: #5a67d8;
}

.recovery-button:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

.action-buttons {
  display: flex;
  gap: 1rem;
}

.logout-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  border: 1px solid #dc2626;
  background: white;
  color: #dc2626;
  border-radius: 6px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.logout-button:hover {
  background: #dc2626;
  color: white;
}

.button-icon {
  width: 1rem;
  height: 1rem;
}

/* Responsive Design */
@media (max-width: 640px) {
  .profile-container {
    padding: 1rem;
  }
  
  .profile-header {
    padding: 1.5rem;
  }
  
  .profile-section {
    padding: 1.5rem;
  }
  
  .info-grid {
    grid-template-columns: 1fr;
  }
  
  .seed-actions {
    flex-direction: column;
  }
  
  .action-buttons {
    flex-direction: column;
  }
}
</style> 