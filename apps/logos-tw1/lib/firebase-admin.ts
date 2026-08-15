import { App, getApps, initializeApp } from "firebase-admin/app";

/**
 * Admin SDK. On App Hosting / Cloud Run, no-arg `initializeApp()` uses
 * application default credentials + FIREBASE_CONFIG.
 *
 * @see https://firebase.google.com/docs/app-hosting/firebase-sdks
 */
export function getAdminApp(): App {
  const existing = getApps()[0];
  if (existing) return existing;
  try {
    return initializeApp();
  } catch {
    return initializeApp({
      projectId: process.env.GCLOUD_PROJECT ?? "tri-weavon",
    });
  }
}
