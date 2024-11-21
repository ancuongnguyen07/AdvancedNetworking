# The exercise environment

## Setting up Mininet

Many of the exercises on this course use the [Mininet
emulator](https://mininet.org/) to build virtual network environments with given
topologies and characteristics. Mininet requires Linux to run. If you do not
have a Linux system available, you will need a virtual machine hosting the Linux
system. VirtualBox is a commonly used, free virtualization environment. See
instructions from the Linux course for more detailed installation instructions
and troubleshooting on installing the virtual environment.

If you are using MacOS with the the ARM processors, UTM (a QEMU installation for
Mac) should work. After installing UTM, you should find the Ubuntu 22.04 image
provided in UTM catalogue, which should work according to instructions in this
chapter. If you are using Windows, you should note that Windows Subsystem for
Linux does not work with mininet and other networking tools used on this course,
but you'll need an actual virtual machine installation (such as VirtualBox).

The course assignments and software are tested on a recent Ubuntu Linux
distribution, but other distributions may work as well. Mininet is mostly
implemented using Python, and we have used Python version 3.12 for testing. The
rest of the instructions assume Ubuntu distribution.

After getting your virtual machine up and running, you should update the Ubuntu
packages and install a few other packages needed by the tools in this course:

    sudo apt-get update
    sudo apt-get upgrade
    sudo apt install git python-is-python3 help2man pip net-tools
    sudo apt install telnet cgroup-tools cgroupfs-mount iputils-ping

You should also set up ssh keys on your virtual machine, that are needed
for git access. New keys can be generated using `ssh-keygen` command on command
line terminal. You can use default options to questions `ssh-keygen` presents.
Copy the public key (`$HOME/.ssh/id_rsa.pub`) to your GitHub settings: "Settings
/ SSH and GPG keys" from the top right corner in GitHub web interface, then
press "New SSH key", and copy the public key to the correct text field. You can
output the key on Linux terminal by typing `cat $HOME/.ssh/id_rsa.pub`.

Clone Mininet from git repository. On this course we use our own fork that has a
few additional scripts and modifications compared to the original parent
repository:

    git clone https://github.com/PasiSa/mininet

After this, install mininet, along with some additional network tools it needs:

    mininet/util/install.sh -fw
    sudo apt-get install openvswitch-switch
    sudo service openvswitch-switch start
    cd mininet
    sudo make install

Now Mininet should work. You can try it using one of our simple network
scripts:

    sudo aalto/simple_topo.py --delay=200ms

The script implements a simple topology with four nodes (titled "lh1", "lh2",
"rh1" and "rh2"), two routers, connected with a bottleneck link that has one-way
propagation latency of 200 ms. Mininet command line interface opens, where you
can run different programs in one of the emulated mininet nodes.

TODO: picture

For example, typing `lh1 ping rh1` starts a ping tool at "lh1" (at IP address
10.0.0.1) that sends ICMP echo requests to "rh1" (at IP address 10.0.0.3), that
replies them. You should see the output on terminal, in most cases reporting a
bit over 400 ms delay in getting responses, because the packets travel through
the bottleneck link that has 200 ms propagation time.

Sometimes it happens that mininet crashes in the middle of simulation, or when
it is starting up. In this case some of the network state may end up in
unfinished state that prevents mininet from being started again. In such
situation you can clean up the network state by typing `sudo mn -c`, and try to
start mininet after that.

## Setting up the course exercise software

Many of the exercises on this course communicate with a tool called
"adnet-agent", that performs different tasks depending on the exercise, and
communicates using a protocol that will be eventually described along with the
assignments. Adnet-agent is implemented in [Rust
language](https://www.rust-lang.org/). Therefore, next you should install the
Rust build tools on your virtual machine by running:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Then follow instructions appearing on terminal.

Clone `adnet-agent` from git repository, for example in your home directory root

    cd $HOME
    git clone git@github.com:PasiSa/adnet-agent.git

Go to the `adnet-agent` directory and build the executable from source:

    cd adnet-agent
    cargo adnet-agent

We will tell more about `adnet-agent` later with the assignments. You can find
the adnet-agent source code in the [git
repository](https://github.com/PasiSa/adnet-agent).

Some of the course assignments involve programming network software. To help you
get started with the assignments, the course material contains code examples
written in the **Rust** language. Recently, Rust has gained popularity among
people working with network software to replace older languages such as C or
C++, for example, due to its properties related to safer memory management.
However, the exercises are designed so that they do not require any particular
programming language. Therefore you can use also C or C++ in the to implement
the exercise assignments. Also Python should work, although it may be a bit more
difficult to operate on binary data with Python, which many of the exercises on
this course require. JavaScript is not a viable choice on this course.

If you are new to Rust, don't be afraid to try it. The Rust development team has
provided comprehensive [online resources](https://www.rust-lang.org/learn) for
learning Rust. You can start, for example, from the [Rust
book](https://doc.rust-lang.org/book/). There are plenty of Rust examples in the
Internet that can be found with some googling. Also ChatGPT knows Rust pretty
well.
