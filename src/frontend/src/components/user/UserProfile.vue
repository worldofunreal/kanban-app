<template>
  <div class="max-w-4xl mx-auto p-6">
    <Card>
      <CardHeader class="text-center bg-gradient-to-r from-primary to-primary/80 text-primary-foreground">
        <CardTitle class="text-3xl">Account Settings</CardTitle>
        <CardDescription class="text-primary-foreground/90">
          Manage your account and preferences
        </CardDescription>
      </CardHeader>

      <CardContent class="p-6 space-y-6">
        <!-- User Information -->
        <div class="space-y-4">
          <h3 class="text-xl font-semibold">Account Information</h3>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-2">
              <Label class="text-sm font-medium text-muted-foreground uppercase tracking-wide">Name</Label>
              <div class="text-sm font-medium">{{ user?.profile?.name || 'Not set' }}</div>
            </div>
            
            <div class="space-y-2">
              <Label class="text-sm font-medium text-muted-foreground uppercase tracking-wide">Principal ID</Label>
              <div class="flex items-center gap-2">
                <div class="text-sm font-mono break-all">{{ principal }}</div>
                <Button
                  variant="ghost"
                  size="icon"
                  class="h-6 w-6"
                  title="Copy Principal"
                  @click="copyPrincipal"
                >
                  <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" />
                    <path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z" />
                  </svg>
                </Button>
              </div>
            </div>
            
            <div class="space-y-2">
              <Label class="text-sm font-medium text-muted-foreground uppercase tracking-wide">Email</Label>
              <div class="text-sm font-medium">{{ user?.profile?.email || 'Not set' }}</div>
            </div>
            
            <div class="space-y-2">
              <Label class="text-sm font-medium text-muted-foreground uppercase tracking-wide">Bio</Label>
              <div class="text-sm font-medium">{{ user?.profile?.bio || 'No bio added' }}</div>
            </div>
          </div>
        </div>

        <div class="border-t my-6"></div>

        <!-- Seed Phrase Recovery -->
        <div class="space-y-4">
          <h3 class="text-xl font-semibold">Account Recovery</h3>
          
          <div class="space-y-6">
            <div class="border-amber-200 bg-amber-50 text-amber-800 p-4 rounded-md">
              <div class="flex items-start gap-3">
                <svg class="h-4 w-4 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
                </svg>
                <div>
                  <h4 class="font-medium">Save Your Seed Phrase</h4>
                  <p class="text-sm mt-1">
                    This is the only way to recover your account. Write it down and keep it safe!
                  </p>
                </div>
              </div>
            </div>
            
            <div class="space-y-4">
              <Label class="text-sm font-medium">Your Seed Phrase (12 words)</Label>
              <div class="p-4 bg-muted border rounded-md font-mono text-sm leading-relaxed">
                <span v-if="showSeedPhrase" class="text-foreground">{{ seedPhrase }}</span>
                <span v-else class="text-muted-foreground tracking-widest">•••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• •••••••••••• ••••••••••••</span>
              </div>
              
              <div class="flex gap-2 flex-wrap">
                <Button 
                  :variant="showSeedPhrase ? 'destructive' : 'outline'"
                  @click="toggleSeedPhrase"
                >
                  {{ showSeedPhrase ? 'Hide Seed Phrase' : 'Show Seed Phrase' }}
                </Button>
                
                <Button 
                  v-if="showSeedPhrase"
                  @click="copySeedPhrase"
                >
                  Copy to Clipboard
                </Button>
              </div>
            </div>
          </div>
        </div>

        <div class="border-t my-6"></div>

        <!-- Account Recovery -->
        <div class="space-y-4">
          <h3 class="text-xl font-semibold">Recover Different Account</h3>
          
          <div class="space-y-4">
            <p class="text-sm text-muted-foreground">
              Enter a different seed phrase to switch to another account:
            </p>
            
            <Textarea
              v-model="recoverySeedPhrase"
              placeholder="Enter 12-word seed phrase..."
              rows="3"
            />
            
            <div v-if="recoveryError" class="border-destructive bg-destructive/10 text-destructive p-3 rounded-md">
              <p class="text-sm">{{ recoveryError }}</p>
            </div>
            
            <Button 
              :disabled="recoveryLoading || !recoverySeedPhrase.trim()"
              @click="handleRecovery"
            >
              {{ recoveryLoading ? 'Recovering...' : 'Recover Account' }}
            </Button>
          </div>
        </div>

        <div class="border-t my-6"></div>

        <!-- Logout -->
        <div class="space-y-4">
          <h3 class="text-xl font-semibold">Account Actions</h3>
          
          <Button variant="outline" class="text-destructive border-destructive hover:bg-destructive hover:text-destructive-foreground" @click="handleLogout">
            <svg class="mr-2 h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M3 3a1 1 0 00-1 1v12a1 1 0 102 0V4a1 1 0 00-1-1zm10.293 9.293a1 1 0 001.414 1.414l3-3a1 1 0 000-1.414l-3-3a1 1 0 10-1.414 1.414L14.586 9H7a1 1 0 100 2h7.586l-1.293 1.293z" clip-rule="evenodd" />
            </svg>
            Logout
          </Button>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { useAuthStore } from '@/stores/auth';
import Card from '../ui/Card.vue';
import CardHeader from '../ui/CardHeader.vue';
import CardTitle from '../ui/CardTitle.vue';
import CardDescription from '../ui/CardDescription.vue';
import CardContent from '../ui/CardContent.vue';
import Button from '../ui/Button.vue';
import Label from '../ui/Label.vue';
import Textarea from '../ui/Textarea.vue';


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

 