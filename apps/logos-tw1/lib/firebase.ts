import { FirebaseApp, getApps, initializeApp } from "firebase/app";

/**
 * Client SDK. On App Hosting, `initializeApp()` with no args reads the
 * FIREBASE_WEBAPP_CONFIG injected at Cloud Build (postinstall). Locally we
 * fall back to project id only — no secrets.
 *
 * @see https://firebase.google.com/docs/app-hosting/firebase-sdks
 */
export function getFirebaseApp(): FirebaseApp {
  const existing = getApps()[0];
  if (existing) return existing;
  try {
    return initializeApp();
  } catch {
    return initializeApp({
      projectId:
        process.env.NEXT_PUBLIC_FIREBASE_PROJECT_ID ?? "tri-weavon",
    });
  }
}
