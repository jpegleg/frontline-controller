# frontline-controller
MDP-like threaded controller daemon with loopback redis storage

Have redis running on the loopback device where the frontline-controller runs.

The policy.toml must be constructed to instruct the controller.

net is a float of max network connections before we decrement the health counter
cpu is a float of max percent CPU (note sysinfo CPU measurements are BROKEN, so we'll sample from /proc/loadavg instead) usage 0.80 for 80%
mem is a float of the max percent RAM (note sufficiently sensitive ) usage 0.75 for 75%
dsk is a float of percentage of the "/" partition/slice disk usage 89.00 for 89%

The url is the fqdn: google.com
the urlp is the fqdn with port: google.com:443
the apiX is the base64 BLAKE2 of the healthy response from the corresponding urlp rustls client connection. The data and heads of the probe can be customized.


This section is where the different APIs have individual health scores. We can do logic based on thresholds reached in these values.
Every time a rustls probe fails or gets anything other than the exact match, the individual score goes down. These values in policy set the "redis key name" of the score modification. 

```
health1 = "health01"
health2 = "health02"
health3 = "health03"
health4 = "health04"
```

So `echo "get health02" | redis-cli` returns 100 if the health score for endpoint 02 is at max. THe health will dip down if probes continue to fail. If we have "flapping", the health score will dip then quickly come back up over and over. Other programs and controlls can also then tap into the data via redis reads, or even writes if we are careful :)

This means we can overlap the scores if we wanted to link the health logic, even between controllers. By default I find having these separate is useful though.

Then the hardest value to set perhaps is the BLAKE2 hashes for the api success:

```
api4 = "tNs4N/1sSyLQ30ffn50KXhHuCQ4DCv55E+BZ2+mwd1+g7eGFUNOiRe/5sjM6d3LrOE6z2guEikkEo8cibWVgcQ=="
```

This value can be gained by having a small program collect these base64 BLAKE2s for when the expected response is returned, the apis.rs file can be used to generate the hashes initially and as response changes are introduced.
Any variable data such as time data will not work for the default logic here, because the value will always change. Because rustls has time data by default, we have a "trim" value coded in that is how many bytes of the response to discard before making the hash. Some responses might need 500 bytes trimmed off to exclude the time data. Be careful not to trim the whole response :)

If we want to use logic on a value that always changes, then we should invert the logic so that we are checking to make sure it is changing and not staying the same.

The STDERR output is the 4 API slots rustls ciphersuites used, while the controller STDOUT has the sampled vs policy comparisons of the net, sys, cpu, dsk, values, and will print audit logging if automation is triggered.

This controller is not meant to run inside containers although could be used that way easily too, instead the general design it is meant to run on the underlying host or VM of an automation node that reacts to repeated error or risk detection and reactions. This template allows us to customize the logic and reactions of the controller, providing an example of having local eactions to clean up disk space and remote IR reactions to SSH and execute API commands. If we run it in a container, we might not have the same local thread logic at all and instead only have API connection logic.

Statically compiling the binary example:
```
docker run -v $PWD:/volume --rm -t clux/muslrust:stable cargo build --release

```

And install:
```
mv target/x86_64-unknown-linux-musl/release/frontline-controller /usr/local/sbin/
```
(or put into an OCI image like a scratch container)


If you don't want the audit data in STDOUT or STDERR, send it to dev null:

```
nohup ./frontline-controller >/dev/null 2>&1 &
```

However having the log file can be useful, especially if the sleep value (SMIG) is more lax. 

```
nohup frontline-controller >> /var/log/audit/controller.log 2>/dev/null &
```

The policy.toml must be in the $pwd of the controller. 

Another value that might be customized at compile time is the SMIG constant. The SMIG constant in frontline-controller is a number of milliseconds to waste between actions to slow down the loops. A smig of 5000 is very lax and a smig of 90 is tight. I'm using between 200 and 2001 currently depending on the situation. Issues less than 2 seconds long might be missed with a SMIG set to 2000, but shorter SMIG will also mean faster health fall rates and more STDOUT and STDERR data.

The logic is to be kept within main.rs. In the main fn we have three threads by default, one for the local system probes, one for the rustls network probes, and one for reactive actions. The reactive actions can write locks in redis that other cause other reactions to cease until a default 9 minute sleep period elapses. It is possible for a score to get extremely low and take more than 9 minutes to get above threshold, resulting in an action happening again. To handle this better we should check by how much the health has improved since last action, and skip action if the improvement is sufficient. The time units are again tuned with SMIGs.

```
            if health2 < 99 {
                println!("{} {} Health2 is at: {} <-+-+-+-<<<", &readu, &uid, health2);
                if health2 < 75 {
                    if cooloff == 0 {
                        println!("{} {} STARTING AUTOMATED RECOVERY b1", &readu, &uid);
                        let _hog = redisset(&"cooloff".to_string(), "1".to_string());
                        let _mop = reactions::b1(&uid);
                        thread::sleep(time::Duration::from_millis(540000));
                        let _out = redisset(&"cooloff".to_string(), "0".to_string());
                    }
                }

            }
```


The reactions.rs contains the reactive functions to execution when thresholds of health score/s are below various levels. We might have 75 set as the threshold for SSHing into an API server or starting to use kubectl against it's control plane. We'll take actions that are known to correct real problems like cluster drift or webserver crashes, as well as common issues like disk space consumption. Having the negative probe result in a -4 to a health score, and a success state resulting in a +1 health to a max of 100. We can then take actions further down the line of events if the score isn't improving and we get to a score of say 0 we can then perhaps change tactics to more drastic measures and lock further reactions, escalating to a human. Anything we want.


## TODO

- error handling
- more logic examples, including the "long lock" and "known progress since" examples.
- improve redis interaction with batch queries (this will probably be one of the next areas worked on)
- add feature for known error state matching
- continued work to eliminate Command calls

The network connections metric, or any of the metrics, can be swapped out for other probes such as inodes or logged in users. The probes for the local system go in locksys.rs. The container version doesn't need locksys.rs or that entire thread and can move to being two threads.
