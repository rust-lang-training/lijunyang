// This module is the CJS entry point for the library.

// The Rust addon.
import * as addon from './load.cjs';

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module './load.cjs' {
    function hello(): string;
    function requestStsToken(
        bucketName: string,
        objectKey: string,
        ttlSeconds?: number
    ): StsToken;
}

export type Greeting = {
    message: string;
};
export type StsToken = { token: string; expiration: number };
export function requestStsToken(
    bucketName: string,
    objectKey: string,
    ttlSeconds?: number
) {
    return addon.requestStsToken(bucketName, objectKey, ttlSeconds);
}

export function greeting(): Greeting {
    const message = addon.hello();
    return { message };
}
