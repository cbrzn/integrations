import { Args_addFile, Args_cat, Args_resolve, Ipfs_Ipfs_ResolveResult } from "./wrap";

export function addFile(_: Args_addFile): String {
  return "";
}

export function cat(_: Args_cat): ArrayBuffer {
  return new ArrayBuffer(0)
}

export function resolve(_: Args_resolve): Ipfs_Ipfs_ResolveResult {
  return {
    cid: "",
    provider: ""
  }
}