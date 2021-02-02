Goal: have this thing run well.. not download deps and rustc and all that on every run.

Goals for docker:

- Have the webserver be built and run with the 'Dockerfile' turned into an image. DONE

- Have the server run on port 80 'in' the container.

- An http request to port 80 on the container should respond with an http response 200

- "Bind" (?) that 80 port on the container to 8080 on host. An http request to port 8080 on the host should respond with http response 200.

- The IP of my laptop + port 8080 should respond with the http response.

DONE!

- The application could have zero runtime deps - with the exception of libc. Libc is a reality. The standard library relies on os libraries that would be very very hard to implement. Not sure exactly why, but those interfaces are in a way, beyond the scope of rust right now. Maybe it would be ideal to have an os where those are written in rust and added as a library if needed.

- How is standard input (stdin), standard output (stdout), and standard error (stderr) dealt with? Those are streams, right? File streams?

- 'stderr' will "pipe" (stream) to an error reporting/recording service.
- 'stdin' includes the environment variables.
- 'stdout' is general information logging.

Ok. This thing is way too big. It should be like 20mb or so. It's like .5gb right now.
