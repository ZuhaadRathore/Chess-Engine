/**
 * Haptic feedback utilities for mobile devices.
 * Provides tactile feedback for user interactions on mobile platforms (iOS and Android).
 * Gracefully fails on desktop or unsupported devices.
 */

let impactFeedback: ((style: 'light' | 'medium' | 'heavy') => Promise<void>) | null = null;
let notificationFeedback: ((type: 'success' | 'warning' | 'error') => Promise<void>) | null = null;
let selectionFeedback: (() => Promise<void>) | null = null;
let isTauri: (() => boolean) | null = null;

// Dynamically import Tauri APIs only if available
const initializeHaptics = async () => {
  try {
    const tauriApi = await import('@tauri-apps/api/core');
    isTauri = tauriApi.isTauri;

    if (isTauri()) {
      const hapticsPlugin = await import('@tauri-apps/plugin-haptics');
      // Wrap the functions to convert Promise<Result> to Promise<void>
      impactFeedback = async (style: 'light' | 'medium' | 'heavy') => {
        await hapticsPlugin.impactFeedback(style);
      };
      notificationFeedback = async (type: 'success' | 'warning' | 'error') => {
        await hapticsPlugin.notificationFeedback(type);
      };
      selectionFeedback = async () => {
        await hapticsPlugin.selectionFeedback();
      };
    }
  } catch (e) {
    // Haptics not available (desktop or plugin not installed)
    console.debug('Haptics not available:', e);
  }
};

// Initialize haptics on module load
initializeHaptics();

/**
 * Trigger impact haptic feedback.
 * Use for: move execution, piece selection, dragging.
 *
 * @param style - The intensity of the impact ('light' | 'medium' | 'heavy')
 * @returns Promise that resolves when haptic feedback is triggered
 *
 * @example
 * await triggerImpact('medium'); // For normal moves
 * await triggerImpact('heavy');  // For captures
 */
export async function triggerImpact(style: 'light' | 'medium' | 'heavy' = 'medium'): Promise<void> {
  if (!isTauri || !isTauri()) {
    return;
  }

  try {
    if (impactFeedback) {
      await impactFeedback(style);
    }
  } catch (e) {
    console.warn('Haptics impact feedback failed:', e);
  }
}

/**
 * Trigger notification haptic feedback.
 * Use for: game end, invalid move, errors.
 *
 * @param type - The type of notification ('success' | 'warning' | 'error')
 * @returns Promise that resolves when haptic feedback is triggered
 *
 * @example
 * await triggerNotification('success'); // Game won
 * await triggerNotification('error');   // Invalid move
 */
export async function triggerNotification(type: 'success' | 'warning' | 'error'): Promise<void> {
  if (!isTauri || !isTauri()) {
    return;
  }

  try {
    if (notificationFeedback) {
      await notificationFeedback(type);
    }
  } catch (e) {
    console.warn('Haptics notification feedback failed:', e);
  }
}

/**
 * Trigger selection haptic feedback.
 * Light feedback for UI interactions.
 * Use for: button presses, square selection, menu navigation.
 *
 * @returns Promise that resolves when haptic feedback is triggered
 *
 * @example
 * await triggerSelection(); // Square selected
 */
export async function triggerSelection(): Promise<void> {
  if (!isTauri || !isTauri()) {
    return;
  }

  try {
    if (selectionFeedback) {
      await selectionFeedback();
    }
  } catch (e) {
    console.warn('Haptics selection feedback failed:', e);
  }
}
