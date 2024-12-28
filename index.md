# Advanced Networking course

This is the course material and assignments for the **Advanced
Networking course (ELEC-E7321)** at Aalto University. Many assignments involve
some network programming tasks. The code examples and assignment templates in
this course are provided in the **[Rust](https://www.rust-lang.org/)** language.
However, you can use also other languages for your implementations and
experimentation, such as C or C++, or even Python. JavaScript is not a viable
option on this course.

_Currently the material is work in progress. You will find more content
appearing constantly in this repository._

The GitHub repository for materials is
[here](https://github.com/PasiSa/AdvancedNetworking). If you find something to
correct or add, feel free to drop an issue or pull request in the repository.

Links to currently available content:

- **[Setting up the exercise environment](environment.md)**. It is recommended
  that you read this with thought already before start of the course. The course
  assignments need to be run in a Linux system with various networking software
  installed. Therefore you should be able to run a virtual machine in your
  computer, hosting the Linux guest with needed software.

- **[Socket programming basics](socket-basics.md)**

- **[Server sockets and parallelism](server-sockets.md)** (_TODO_)

- **[Linux networking](linux-tcpip.md)** (_TODO_)

## Assignments

The assignment descriptions and other possible files needed for assignments are
under the
[assignments](https://github.com/PasiSa/AdvancedNetworking/tree/main/assignments)
folder in this git repository. The assignments also contain program templates
implemented in Rust that can be used to help you to get started with the
assignment. You may use it or implement your own solution from scratch.

One option is to clone or fork this repository to your local system, after which
you can start modifying the provided assignment templates, and maintain your
work in a forked personal git repository. This makes it easier to synchronize
your modifications between different systems, for example if you want to develop
you assignment code in your native system and development tools, but run the
code in the virtual Linux guest, that is technically a different machine in your
system.

The assignments are as follows:

- [Task 1: Simple client](assignments/task-001/task-001.md)
- [Task 2: TCP server](assignments/task-002/task-002.md)
- [Task 3: Data transfer using UDP](assignments/task-003/task-003.md)
- _Task 4: Congestion control and ECN (planned / TODO)_
- _Task 5: HTTP/3 and QUIC (planned / TODO)_
- _Maybe some more..._
