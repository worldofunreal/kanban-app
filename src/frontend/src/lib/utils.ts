import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"
 
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

// Generate auto username for guest accounts
export function generateGuestUsername(): string {
  const randomDigits = Math.floor(Math.random() * 9000) + 1000; // 1000-9999
  return `Guest${randomDigits}`;
} 