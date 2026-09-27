import { Platform } from 'react-native';
import * as LocalAuthentication from 'expo-local-authentication';
import { requireStepUpAuthentication } from '../biometric-lock';

jest.mock('react-native', () => ({
  Platform: { OS: 'ios' },
}));

jest.mock('expo-local-authentication', () => ({
  authenticateAsync: jest.fn(),
}));

describe('biometric-lock step-up mechanism', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  it('bypasses authentication on web platform', async () => {
    Platform.OS = 'web';
    const result = await requireStepUpAuthentication('Test', 300000);
    expect(result).toBe(true);
    expect(LocalAuthentication.authenticateAsync).not.toHaveBeenCalled();
  });

  it('prompts for authentication if outside grace period', async () => {
    Platform.OS = 'ios';
    (LocalAuthentication.authenticateAsync as jest.Mock).mockResolvedValueOnce({ success: true });
    const result = await requireStepUpAuthentication('Test', 300000);
    expect(result).toBe(true);
    expect(LocalAuthentication.authenticateAsync).toHaveBeenCalledTimes(1);
    expect(LocalAuthentication.authenticateAsync).toHaveBeenCalledWith(
      expect.objectContaining({
        promptMessage: 'Test',
        fallbackLabel: 'Use Passcode',
        disableDeviceFallback: false,
      })
    );
  });

  it('respects the configurable grace period for subsequent calls', async () => {
    Platform.OS = 'ios';
    (LocalAuthentication.authenticateAsync as jest.Mock).mockResolvedValueOnce({ success: true });

    // First call authenticates
    await requireStepUpAuthentication('Test 1', 300000);
    expect(LocalAuthentication.authenticateAsync).toHaveBeenCalledTimes(1);

    // Advance time slightly, well within 5 mins
    jest.advanceTimersByTime(60000); // 1 min

    // Second call skips auth because of grace period
    const result = await requireStepUpAuthentication('Test 2', 300000);
    expect(result).toBe(true);
    expect(LocalAuthentication.authenticateAsync).toHaveBeenCalledTimes(1); // Still 1

    // Advance time past the grace period
    jest.advanceTimersByTime(300000); // + 5 mins (total 6 mins)

    (LocalAuthentication.authenticateAsync as jest.Mock).mockResolvedValueOnce({ success: true });
    // Third call should authenticate again
    await requireStepUpAuthentication('Test 3', 300000);
    expect(LocalAuthentication.authenticateAsync).toHaveBeenCalledTimes(2);
  });

  it('returns false if authentication is cancelled or fails', async () => {
    Platform.OS = 'ios';
    (LocalAuthentication.authenticateAsync as jest.Mock).mockResolvedValueOnce({ success: false, error: 'user_cancel' });

    // Reset the module state (timer) by advancing past any previous successful auth
    jest.advanceTimersByTime(1000000);

    const result = await requireStepUpAuthentication('Test', 300000);
    expect(result).toBe(false);
  });
});
