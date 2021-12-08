Message Passing
===============

**TRD:** TBD<br/>
**Working Group:** Kernel<br/>
**Type:** Documentary<br/>
**Status:** Draft<br/>
**Authors:** Alexandru Radovici<br/>

Abstract
-------------------------------

This document describes the design and implementation of the 
message passing (MP) mechanism used by the processes running on top
of the Tock operating system to send and receives messages between
each other. This document is in full compliance with [TRD1].

1 Introduction
====================================================================
Processes running on top of the Tock operating system need a way
to communicate with each other in a secure and reliably manner.
Examples of use cases are:
- user space drivers
- services


2 Terminology
====================================================================
This document uses the following terms when describing the message
passing mechanism.

**Sender**: a process that sends a message to another process.

**Receiver**: a process that receives a message from another process.

**Message**: the data exchange between two processes. This MUST contain
a *Message Header* and MAY contain a *Message Payload*. The message has
exatly one *Sender* and one *Receiver*.

**Message Header**: a set of two `usize` values that are always present
within the message.

**Message Payload**: an optional buffer that is attached to 
a *Message*. The contents of the buffer is opaque to the kernel and
to the message passing mechanism.

**Request**: a *Message* sent by a process to another process.

**Response**: a *Message* sent by a process to another process following 
a *Request*.

**Message Status**: describes the status of a *Message*

**Send**: a message that has been sent but has not yet been read by the
recipient process.
 **Acknowleged**: a *Message* for which the recipient process has read 
the *Message Header*.
**Read**: a message that has been fully read by the recipient process.
If the *Message* had a *Message Payload*, this has been fully copied to the
recipients process memory.

3 Limitations
====================================================================

4 Architecture
====================================================================

5 Reference
====================================================================

6 Acknowledgments
====================================================================


7 Author's Address
====================================================================

    Philip Levis
    409 Gates Hall
    Stanford University
    Stanford, CA 94305

    phone - +1 650 725 9046

    email - pal@cs.stanford.edu

[TRD1]: trd1-trds.md "Tock Reference Document (TRD) Structure and Keywords"
