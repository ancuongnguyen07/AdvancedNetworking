// Daytime server example, taken and modified from W.R. Stevens'
// Unix Network Programming book

#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <time.h>
#include <unistd.h>

#define MAXLINE 1024


int main(int argc, char **argv){

	int listenfd;
	int connfd;
	socklen_t len;
	struct sockaddr_in servaddr, cliaddr;
	char buff[MAXLINE];
	time_t ticks;

    // Requires port number as a command line argument
    if (argc != 2) {
        fprintf(stderr, "usage: daytime-serv <port>\n");
        return 1;
    }

    int port;
    if (sscanf(argv[1], "%d", &port) < 1) {
        fprintf(stderr, "invalid port number\n");
        return 1;
    }

	listenfd = socket(AF_INET, SOCK_STREAM, 0);
	
	bzero(&servaddr,sizeof(servaddr));
	servaddr.sin_family = AF_INET;
	servaddr.sin_addr.s_addr = htonl(INADDR_ANY);
	servaddr.sin_port = htons(port);

	bind(listenfd, (struct sockaddr *) &servaddr, sizeof(servaddr));

	listen(listenfd, 10);  // listen Queue for 10 pending connections

	while(1) {

		len = sizeof(cliaddr);
		// active socket is created at connfd
		connfd = accept(listenfd, (struct sockaddr *) &cliaddr, &len);
		printf("connection from %s, port %d\n",
		    inet_ntop(AF_INET, &cliaddr.sin_addr, buff, sizeof(buff)),
		    ntohs(cliaddr.sin_port));

		ticks = time(NULL);
		snprintf(buff, sizeof(buff), "%.24s\r\n", ctime(&ticks));
		write(connfd, buff, strlen(buff));

		close(connfd); // closes the active socket, not listening socket
	}
}