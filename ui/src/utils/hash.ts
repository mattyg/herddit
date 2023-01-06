import {isEqual} from "lodash";

export const areHashesEqual = (hash1: Uint8Array, hash2: Uint8Array) => isEqual(hash1, hash2);