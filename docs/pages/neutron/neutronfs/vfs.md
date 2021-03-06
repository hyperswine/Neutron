---
layout: default
title: VFS
parent: Neutron
---

## Overview

The VFS is a layer is specifically integrated with NeFS semantics. Think of the VFS as an in-memory view of NeFS.

The key structure is the vnode. This represents an inode, which is either a file, dir, device, link, etc. The vnode is the atomic unit of the VFS. And it can be opened, closed, read, written.

The VFS implements `stat`, `open`, `chmod`, etc.

## Superblock

Like a fs, the VFS itself has a superblock that stores all the metadata on a specific filesystem. It exposes methods that can be used to manipulate it, like allocating inodes, destroying inodes, etc.

```rust
struct VFSSuperBlock;

trait VFSFunctions {
    // lifecycle
    fn alloc_inode();
    fn dealloc_inode();
    // management
    fn find_inode(InodeNumber) -> Inode;
    fn sync_fs();
}
```

## Vnodes and Files

Vnodes represent inodes whereas files are the unit of operation for most things.
