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
of the Tock operating system. Processes use it to send and receives 
messages between each other. This document is in full compliance with [TRD1].

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
a *Header* and MAY contain a *Payload*. The message has
exactly one *Sender* and one *Receiver*.

**Header**: a set of two `usize` values that are always present
within the message. The two `usize` values correspond to the two arguments
of the `command` system call.

**Payload**: an optional buffer that is attached to 
a *Message*. The contents of the buffer is opaque to the kernel and
to the message passing mechanism.

**Request**: a *Message* sent by a *Sender* to a *Receiver*.

**Response**: a *Message* sent by a *Receiver* to a *Sender* following 
a *Request*.

**Message Status**: describes the status of a *Message*.

**Sent**: a message that has been sent but has not yet been read by the
*Receiver*.
 **Acknowleged**: a *Message* for which the *Receiver* has read 
the *Message Header*.
**Read**: a message that has been fully read by the *Receiver*.
If the *Message* had a *Message Payload*, this has been fully copied to the
recipients process memory.

3 Limitations
====================================================================
The message passing mechanisms has a few limitations.

3.1 Number of senders and receivers
-----------------------------------
The maximum number of processes that it can serve is 32. This design decision
is based upon two idea:
  - systems running Tock will most probably run only a handful of processes
  - limiting the number of processes to 32 allows the kernel to use primitive values
  to notify processes about their messages states.

NOTE: A Tock system might run more than 32 processes, but only the first 32 of them
that are involved with the message passing will be served.

3.2 Messages
-----------------------------------
There can be only a maximum of one *Message* in transit between a *Sender*
and a *Receiver*. This decission has been made as the Tock kernel is
not allowed to allocate memory dynamically, and implementing a message
queue would require this.

NOTE: The user space message passing library MIGHT implement a message queue.

4 Message
====================================================================

5 Architecture
====================================================================

The message passing mechanism assumes that memory cannot be shared 
between processes. This allows it to properly work on any hardware,
regardless of the restrictions imposed by the hardware memory protection
system.

5.x Message Format
--------------------

Each *Messages* is composed out of two parts:
 - *Header* which consists of two `usize` values
 - *Payload* which is optional and is represented by a buffer
 - *

```
+---------+---------+
| Value 1 | Value 2 |
+---------+---------+
```



For sending an receiving messages, each process has a number of 
64 read write allowed buffers devided into two cathegories:
  - *Sending Buffers* that hold the *Payload* for 
    - the *Request* messages (in transit messages of the *Sender*)
    - the *Response* messages (in transit reply messages from the *Receiver*)
  - *Receiving Buffers* that hold the *Payload* for *Requests* that are read

+--------+   Send    +------------+   Notify    +----------+
| Sender | --------> | MP Capsule | ----------> | Receiver |
+--------+           +------------+      Ack    +----------+
                                    <----------           


Buffers
-------
Sending (RW) 0..31
Receiving (RW) 32..63

1. -----> Send
2. <----- Ack
3  <----- Read
4  <----- Reply

5  -----> Read Reply


6 Reference
====================================================================

7 Acknowledgments
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
