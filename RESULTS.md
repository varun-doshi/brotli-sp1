## Results

### Running Default SP1 Proof
Time taken: 425540ms = 7.09 mins
```
cd ./script && RUST_LOG=info cargo run --release -- --prove
```

Output:
```
n: 000b438086038469752135820422b87b000402f87683064aba80840bebc200840bebc2008401c9c3809407620929b4c0fc48060c4faf378b1e85681c9de1872386f26fc1000080c001a0a47813853bd8456a2f545cffbbd8d6be8292f7e43d6f75cbb61b242ec0a60ab9a0568dd62109b55d483e6407598a69ca62a2e77d25eea4c192962f71eb1ad9c47203
2026-02-17T07:31:58.343024Z  INFO submitting task of kind SetupVkey
2026-02-17T07:31:58.994395Z  INFO Setup completed for task local_worker_01khn86nr7fcr9pgktsdn4pns9
2026-02-17T07:31:58.994430Z  INFO Upload completed for artifact artifact_01khn86nr6ecrrtjmkn09v4akg
2026-02-17T07:31:58.994978Z  INFO submitting task of kind Controller
2026-02-17T07:31:58.995063Z  INFO submitting task of kind SetupVkey
2026-02-17T07:31:59.617156Z  INFO Setup completed for task local_worker_01khn86pckfp9ae73ywmsfqr1n
2026-02-17T07:31:59.617191Z  INFO Upload completed for artifact artifact_01khn86pckfp9ae73yvn7jvxt4
2026-02-17T07:31:59.619335Z  INFO minimal executor: Starting minimal executor
stdout: Sequencer message length: "000b438086038469752135820422b87b000402f87683064aba80840bebc200840bebc2008401c9c3809407620929b4c0fc48060c4faf378b1e85681c9de1872386f26fc1000080c001a0a47813853bd8456a2f545cffbbd8d6be8292f7e43d6f75cbb61b242ec0a60ab9a0568dd62109b55d483e6407598a69ca62a2e77d25eea4c192962f71eb1ad9c47203"
2026-02-17T07:31:59.652574Z  INFO minimal executor: Minimal Executor finished. elapsed: 0.033215459s, mhz: 192.46321419192188
2026-02-17T07:31:59.652764Z  INFO minimal executor:splicing trace chunk:SplicingVM::new: close time.busy=65.3µs time.idle=291ns
2026-02-17T07:31:59.674562Z  INFO minimal executor:get splices to serialize: submitting task of kind MarkerDeferredRecord
2026-02-17T07:31:59.679960Z  INFO minimal executor:get splices to serialize: submitting task of kind ProveShard
2026-02-17T07:31:59.693932Z  INFO minimal executor:get splices to serialize: submitting task of kind MarkerDeferredRecord
2026-02-17T07:31:59.699441Z  INFO minimal executor:get splices to serialize: submitting task of kind ProveShard
2026-02-17T07:31:59.701530Z  INFO allocating record: close time.busy=482µs time.idle=3.04µs
2026-02-17T07:31:59.715183Z  INFO minimal executor:get splices to serialize: submitting task of kind MarkerDeferredRecord
2026-02-17T07:31:59.720053Z  INFO minimal executor:get splices to serialize: submitting task of kind MarkerDeferredRecord
2026-02-17T07:31:59.720800Z  INFO minimal executor:get splices to serialize: submitting task of kind ProveShard
2026-02-17T07:31:59.721697Z  INFO minimal executor:get splices to serialize: submitting task of kind ProveShard
2026-02-17T07:31:59.721775Z  INFO allocating record: close time.busy=154µs time.idle=1.17µs
2026-02-17T07:31:59.722452Z  INFO minimal executor:splicing trace chunk: close time.busy=69.8ms time.idle=6.29µs
2026-02-17T07:31:59.723470Z  INFO minimal executor:get splices to serialize: close time.busy=851µs time.idle=70.0ms
2026-02-17T07:31:59.723484Z  INFO minimal executor:spawn prove shard tasks: close time.busy=25.7µs time.idle=70.8ms
2026-02-17T07:31:59.723489Z  INFO minimal executor: close time.busy=51.7µs time.idle=104ms
2026-02-17T07:31:59.728958Z  INFO allocating record: close time.busy=383µs time.idle=1.21µs
2026-02-17T07:31:59.744067Z  INFO allocating record: close time.busy=481µs time.idle=1.96µs
2026-02-17T07:32:00.196802Z  INFO submitting task of kind ProveShard
2026-02-17T07:32:00.197651Z  INFO generate dependencies: close time.busy=438ms time.idle=31.7µs
2026-02-17T07:32:00.199178Z  INFO submitting task of kind ProveShard
2026-02-17T07:32:00.204423Z  INFO submitting task of kind ProveShard
2026-02-17T07:32:02.182887Z  INFO generate dependencies: close time.busy=2.19s time.idle=34.1µs
2026-02-17T07:32:02.189082Z  INFO generate dependencies: close time.busy=2.31s time.idle=27.7µs
2026-02-17T07:32:05.292892Z  INFO generate dependencies: close time.busy=5.33s time.idle=25.7µs
2026-02-17T07:32:16.899404Z  INFO generate GKR circuit: close time.busy=1.97s time.idle=701µs
2026-02-17T07:32:21.279166Z  INFO prove GKR circuit: close time.busy=4.38s time.idle=12.4µs
2026-02-17T07:32:22.851152Z  WARN Memory usage is high: 81.64%, we recommend using the prover network
2026-02-17T07:32:32.911406Z  WARN Memory usage is high: 90.35%, we recommend using the prover network
2026-02-17T07:32:42.703381Z  INFO compute jagged values: close time.busy=278ms time.idle=381µs
2026-02-17T07:32:42.927103Z  WARN Memory usage is high: 86.34%, we recommend using the prover network
2026-02-17T07:32:46.174988Z  INFO generate GKR circuit: close time.busy=14.4s time.idle=3.16ms
2026-02-17T07:32:52.930492Z  WARN Memory usage is high: 87.98%, we recommend using the prover network
2026-02-17T07:33:02.957277Z  WARN Memory usage is high: 89.49%, we recommend using the prover network
2026-02-17T07:33:13.060172Z  WARN Memory usage is high: 92.18%, we recommend using the prover network
2026-02-17T07:33:23.105840Z  WARN Memory usage is high: 81.66%, we recommend using the prover network
2026-02-17T07:33:25.714059Z  INFO generate GKR circuit: close time.busy=25.9s time.idle=14.3µs
2026-02-17T07:33:25.921165Z  INFO generate GKR circuit: close time.busy=7.41s time.idle=15.5µs
2026-02-17T07:33:26.000790Z  INFO prove GKR circuit: close time.busy=39.8s time.idle=10.8µs
2026-02-17T07:33:34.046497Z  INFO get recursion program: close time.busy=0.00ns time.idle=93.8s
2026-02-17T07:33:35.576371Z  INFO generate dependencies: close time.busy=1.36s time.idle=4.13ms
2026-02-17T07:33:42.996083Z  INFO compute jagged values: close time.busy=1.99s time.idle=374µs
2026-02-17T07:33:43.128707Z  WARN Memory usage is high: 91.18%, we recommend using the prover network
2026-02-17T07:33:53.145683Z  WARN Memory usage is high: 80.01%, we recommend using the prover network
2026-02-17T07:34:03.151493Z  WARN Memory usage is high: 87.70%, we recommend using the prover network
2026-02-17T07:34:07.526243Z  INFO generate GKR circuit: close time.busy=11.9s time.idle=26.7µs
2026-02-17T07:34:13.152868Z  WARN Memory usage is high: 86.26%, we recommend using the prover network
2026-02-17T07:34:23.154885Z  WARN Memory usage is high: 86.07%, we recommend using the prover network
2026-02-17T07:34:33.180967Z  WARN Memory usage is high: 89.45%, we recommend using the prover network
2026-02-17T07:34:36.683188Z  INFO prove GKR circuit: close time.busy=70.8s time.idle=8.46µs
2026-02-17T07:34:49.214226Z  INFO compute jagged values: close time.busy=1.40s time.idle=9.58µs
2026-02-17T07:34:51.125735Z  INFO prove GKR circuit: close time.busy=85.4s time.idle=8.46µs
2026-02-17T07:34:54.756122Z  INFO prove GKR circuit: close time.busy=47.2s time.idle=1.82ms
2026-02-17T07:34:59.688332Z  INFO get recursion program: close time.busy=0.00ns time.idle=178s
2026-02-17T07:35:03.195679Z  WARN Memory usage is high: 86.45%, we recommend using the prover network
2026-02-17T07:35:06.899250Z  INFO generate dependencies: close time.busy=7.03s time.idle=902µs
2026-02-17T07:35:13.202358Z  WARN Memory usage is high: 81.81%, we recommend using the prover network
2026-02-17T07:35:23.249549Z  WARN Memory usage is high: 85.80%, we recommend using the prover network
2026-02-17T07:35:26.127834Z  INFO compute jagged values: close time.busy=6.16s time.idle=68.6µs
2026-02-17T07:35:33.305952Z  WARN Memory usage is high: 85.82%, we recommend using the prover network
2026-02-17T07:35:37.986672Z  INFO compute jagged values: close time.busy=10.3s time.idle=15.6µs
2026-02-17T07:35:43.333155Z  WARN Memory usage is high: 82.25%, we recommend using the prover network
2026-02-17T07:36:03.348622Z  WARN Memory usage is high: 89.46%, we recommend using the prover network
2026-02-17T07:36:13.353795Z  WARN Memory usage is high: 86.07%, we recommend using the prover network
2026-02-17T07:36:33.359299Z  WARN Memory usage is high: 84.14%, we recommend using the prover network
2026-02-17T07:36:43.364394Z  WARN Memory usage is high: 80.10%, we recommend using the prover network
2026-02-17T07:36:53.369504Z  WARN Memory usage is high: 89.20%, we recommend using the prover network
2026-02-17T07:36:59.285942Z  INFO generate GKR circuit: close time.busy=4.72s time.idle=2.77ms
2026-02-17T07:37:00.612725Z  INFO get recursion program: close time.busy=0.00ns time.idle=295s
2026-02-17T07:37:03.028667Z  INFO generate dependencies: close time.busy=2.00s time.idle=458µs
2026-02-17T07:37:08.492768Z  INFO prove GKR circuit: close time.busy=9.21s time.idle=5.12µs
2026-02-17T07:37:08.948635Z  INFO get recursion program: close time.busy=0.00ns time.idle=307s
2026-02-17T07:37:13.381222Z  WARN Memory usage is high: 84.68%, we recommend using the prover network
2026-02-17T07:37:23.385057Z  WARN Memory usage is high: 87.47%, we recommend using the prover network
2026-02-17T07:37:33.455860Z  WARN Memory usage is high: 89.84%, we recommend using the prover network
2026-02-17T07:37:46.561321Z  INFO compute jagged values: close time.busy=1.13s time.idle=10.2µs
2026-02-17T07:37:54.514763Z  INFO get recursion program: close time.busy=0.00ns time.idle=259s
2026-02-17T07:38:03.496740Z  WARN Memory usage is high: 85.65%, we recommend using the prover network
2026-02-17T07:38:27.721719Z  INFO generate GKR circuit: close time.busy=5.15s time.idle=7.00µs
2026-02-17T07:38:29.123345Z  INFO get recursion program: close time.busy=0.00ns time.idle=202s
2026-02-17T07:38:30.818700Z  INFO prove GKR circuit: close time.busy=3.10s time.idle=958ns
2026-02-17T07:38:40.440303Z  INFO compute jagged values: close time.busy=1.76s time.idle=1.88µs
2026-02-17T07:38:43.519330Z  WARN Memory usage is high: 83.36%, we recommend using the prover network
2026-02-17T07:39:03.564123Z  INFO get recursion program: close time.busy=0.00ns time.idle=121s
Successfully generated proof!
Successfully verified proof!
Proof generation time: 425540ms
```

## Running Groth16 Proof

Time taken: n: 3052225ms = 50.8 mins

```
n: 000b438086038469752135820422b87b000402f87683064aba80840bebc200840bebc2008401c9c3809407620929b4c0fc48060c4faf378b1e85681c9de1872386f26fc1000080c001a0a47813853bd8456a2f545cffbbd8d6be8292f7e43d6f75cbb61b242ec0a60ab9a0568dd62109b55d483e6407598a69ca62a2e77d25eea4c192962f71eb1ad9c47203
2026-02-17T07:56:21.159870Z  INFO submitting task of kind SetupVkey
2026-02-17T07:56:21.777213Z  INFO Setup completed for task local_worker_01khn9ka97fd091ke2m684aqef
2026-02-17T07:56:21.777261Z  INFO Upload completed for artifact artifact_01khn9ka97fd091ke2k8g6wh7c
2026-02-17T07:56:21.777747Z  INFO submitting task of kind Controller
2026-02-17T07:56:21.777807Z  INFO submitting task of kind SetupVkey
2026-02-17T07:56:22.394478Z  INFO Setup completed for task local_worker_01khn9kawhfqr8s63s1v66nf4h
2026-02-17T07:56:22.394518Z  INFO Upload completed for artifact artifact_01khn9kawhfqr8s63rzxradnvr
2026-02-17T07:56:22.396711Z  INFO minimal executor: Starting minimal executor
stdout: Sequencer message length: "000b438086038469752135820422b87b000402f87683064aba80840bebc200840bebc2008401c9c3809407620929b4c0fc48060c4faf378b1e85681c9de1872386f26fc1000080c001a0a47813853bd8456a2f545cffbbd8d6be8292f7e43d6f75cbb61b242ec0a60ab9a0568dd62109b55d483e6407598a69ca62a2e77d25eea4c192962f71eb1ad9c47203"
2026-02-17T07:56:22.430422Z  INFO minimal executor: Minimal Executor finished. elapsed: 0.033690292s, mhz: 189.75062608540173
2026-02-17T07:56:22.430609Z  INFO minimal executor:splicing trace chunk:SplicingVM::new: close time.busy=56.4µs time.idle=291ns
2026-02-17T07:56:22.452948Z  INFO minimal executor:get splices to serialize: submitting task of kind MarkerDeferredRecord
2026-02-17T07:56:22.458428Z  INFO minimal executor:get splices to serialize: submitting task of kind ProveShard
2026-02-17T07:56:22.472859Z  INFO minimal executor:get splices to serialize: submitting task of kind MarkerDeferredRecord
2026-02-17T07:56:22.478475Z  INFO minimal executor:get splices to serialize: submitting task of kind ProveShard
2026-02-17T07:56:22.480294Z  INFO allocating record: close time.busy=396µs time.idle=1.54µs
2026-02-17T07:56:22.494501Z  INFO minimal executor:get splices to serialize: submitting task of kind MarkerDeferredRecord
2026-02-17T07:56:22.498509Z  INFO minimal executor:get splices to serialize: submitting task of kind MarkerDeferredRecord
2026-02-17T07:56:22.500311Z  INFO minimal executor:get splices to serialize: submitting task of kind ProveShard
2026-02-17T07:56:22.500351Z  INFO minimal executor:splicing trace chunk: close time.busy=69.8ms time.idle=7.21µs
2026-02-17T07:56:22.500636Z  INFO minimal executor:get splices to serialize: submitting task of kind ProveShard
2026-02-17T07:56:22.501226Z  INFO minimal executor:get splices to serialize: close time.busy=1.50ms time.idle=69.3ms
2026-02-17T07:56:22.501261Z  INFO minimal executor:spawn prove shard tasks: close time.busy=102µs time.idle=70.7ms
2026-02-17T07:56:22.501284Z  INFO minimal executor: close time.busy=88.6µs time.idle=105ms
2026-02-17T07:56:22.502218Z  INFO allocating record: close time.busy=521µs time.idle=1.54µs
2026-02-17T07:56:22.508551Z  INFO allocating record: close time.busy=471µs time.idle=2.04µs
2026-02-17T07:56:22.525378Z  INFO allocating record: close time.busy=546µs time.idle=1.42µs
2026-02-17T07:56:22.834497Z  INFO submitting task of kind ProveShard
2026-02-17T07:56:22.836134Z  INFO submitting task of kind ProveShard
2026-02-17T07:56:22.837588Z  INFO submitting task of kind ProveShard
2026-02-17T07:56:22.837630Z  INFO All core proofs received: number of core proofs: 7
2026-02-17T07:56:22.837639Z  INFO Number of core proofs completed: 7
2026-02-17T07:56:22.886419Z  INFO generate dependencies: close time.busy=344ms time.idle=25.6µs
2026-02-17T07:56:23.002874Z  INFO get recursion program: close time.busy=116ms time.idle=345µs
2026-02-17T07:56:25.775555Z  INFO generate dependencies: close time.busy=3.10s time.idle=92.8µs
2026-02-17T07:56:25.785238Z  INFO generate dependencies: close time.busy=3.20s time.idle=24.6µs
2026-02-17T07:56:25.804959Z  INFO generate dependencies: close time.busy=3.13s time.idle=3.08ms
2026-02-17T07:56:25.963685Z  INFO get recursion program: close time.busy=173ms time.idle=53.3µs
2026-02-17T07:56:25.966130Z  INFO get recursion program: close time.busy=181ms time.idle=2.85ms
2026-02-17T07:56:25.969028Z  INFO get recursion program: close time.busy=154ms time.idle=8.32ms
2026-02-17T07:56:36.389908Z  WARN Memory usage is high: 82.55%, we recommend using the prover network
2026-02-17T07:56:42.547507Z  INFO generate GKR circuit: close time.busy=2.29s time.idle=18.0µs
2026-02-17T07:56:46.396166Z  WARN Memory usage is high: 83.56%, we recommend using the prover network
2026-02-17T07:56:56.401734Z  WARN Memory usage is high: 81.78%, we recommend using the prover network
2026-02-17T07:57:06.428763Z  WARN Memory usage is high: 89.28%, we recommend using the prover network
2026-02-17T07:57:13.169480Z  INFO prove GKR circuit: close time.busy=30.6s time.idle=11.5µs
2026-02-17T07:57:16.455195Z  WARN Memory usage is high: 88.85%, we recommend using the prover network
2026-02-17T07:57:16.747916Z  INFO generate GKR circuit: close time.busy=11.3s time.idle=130µs
2026-02-17T07:57:16.793717Z  INFO generate GKR circuit: close time.busy=11.8s time.idle=2.72ms
2026-02-17T07:57:20.446312Z  INFO compute jagged values: close time.busy=203ms time.idle=3.33µs
2026-02-17T07:57:35.778645Z  INFO generate GKR circuit: close time.busy=3.99s time.idle=8.58µs
2026-02-17T07:57:36.461743Z  WARN Memory usage is high: 80.44%, we recommend using the prover network
2026-02-17T07:57:43.633162Z  INFO prove GKR circuit: close time.busy=26.9s time.idle=1.00µs
2026-02-17T07:57:52.042883Z  INFO generate GKR circuit: close time.busy=5.37s time.idle=3.50µs
2026-02-17T07:57:57.909797Z  INFO compute jagged values: close time.busy=1.20s time.idle=2.54µs
2026-02-17T07:58:00.155358Z  INFO prove GKR circuit: close time.busy=43.4s time.idle=3.12µs
2026-02-17T07:58:03.147009Z  INFO prove GKR circuit: close time.busy=11.1s time.idle=1.79µs
2026-02-17T07:58:08.284524Z  INFO prove GKR circuit: close time.busy=32.5s time.idle=3.25µs
2026-02-17T07:58:19.324264Z  INFO compute jagged values: close time.busy=5.36s time.idle=9.88µs
2026-02-17T07:58:26.486802Z  WARN Memory usage is high: 89.49%, we recommend using the prover network
2026-02-17T07:58:35.347490Z  INFO compute jagged values: close time.busy=1.33s time.idle=7.25µs
2026-02-17T07:58:36.492390Z  WARN Memory usage is high: 80.16%, we recommend using the prover network
2026-02-17T07:58:43.957255Z  INFO compute jagged values: close time.busy=2.28s time.idle=8.21µs
2026-02-17T07:58:46.494012Z  WARN Memory usage is high: 81.50%, we recommend using the prover network
2026-02-17T07:58:49.434139Z  INFO generate dependencies: close time.busy=1.48s time.idle=2.00ms
2026-02-17T07:58:49.682730Z  INFO get recursion program: close time.busy=248ms time.idle=60.6µs
2026-02-17T07:59:16.498101Z  WARN Memory usage is high: 89.18%, we recommend using the prover network
2026-02-17T07:59:26.498580Z  WARN Memory usage is high: 86.69%, we recommend using the prover network
2026-02-17T07:59:31.185021Z  INFO generate GKR circuit: close time.busy=3.07s time.idle=259µs
2026-02-17T07:59:34.717293Z  INFO prove GKR circuit: close time.busy=3.53s time.idle=1.42µs
2026-02-17T07:59:46.505503Z  WARN Memory usage is high: 85.71%, we recommend using the prover network
2026-02-17T07:59:51.185982Z  INFO compute jagged values: close time.busy=138ms time.idle=7.71µs
2026-02-17T07:59:57.460444Z  INFO generate GKR circuit: close time.busy=1.03s time.idle=2.62µs
2026-02-17T08:00:06.192226Z  INFO No neighboring range found, adding proof to tree
2026-02-17T08:00:08.575114Z  INFO generate dependencies: close time.busy=2.12s time.idle=813µs
2026-02-17T08:00:08.776921Z  INFO get recursion program: close time.busy=202ms time.idle=42.3µs
2026-02-17T08:00:11.333092Z  INFO prove GKR circuit: close time.busy=13.9s time.idle=1.25µs
2026-02-17T08:00:15.609509Z  INFO generate GKR circuit: close time.busy=10.5s time.idle=6.96µs
2026-02-17T08:00:16.522834Z  WARN Memory usage is high: 81.32%, we recommend using the prover network
2026-02-17T08:00:17.053753Z  INFO generate GKR circuit: close time.busy=1.89s time.idle=12.1µs
2026-02-17T08:00:17.680042Z  INFO compute jagged values: close time.busy=517ms time.idle=8.75µs
2026-02-17T08:00:28.929896Z  INFO generate GKR circuit: close time.busy=2.21s time.idle=33.1µs
2026-02-17T08:00:31.154841Z  INFO prove GKR circuit: close time.busy=14.1s time.idle=1.29µs
2026-02-17T08:00:31.723717Z  INFO No neighboring range found, adding proof to tree
2026-02-17T08:00:31.786401Z  INFO prove GKR circuit: close time.busy=16.2s time.idle=7.21µs
2026-02-17T08:00:34.128213Z  INFO generate dependencies: close time.busy=2.32s time.idle=923µs
2026-02-17T08:00:34.131717Z  INFO get recursion program: close time.busy=19.3µs time.idle=2.18ms
2026-02-17T08:00:46.053248Z  INFO compute jagged values: close time.busy=702ms time.idle=9.79µs
2026-02-17T08:00:50.597661Z  INFO compute jagged values: close time.busy=2.87s time.idle=18.5µs
2026-02-17T08:00:52.433492Z  INFO prove GKR circuit: close time.busy=23.5s time.idle=2.71µs
2026-02-17T08:00:56.545610Z  WARN Memory usage is high: 81.28%, we recommend using the prover network
2026-02-17T08:01:06.555810Z  WARN Memory usage is high: 89.45%, we recommend using the prover network
2026-02-17T08:01:07.074579Z  INFO compute jagged values: close time.busy=2.17s time.idle=17.0µs
2026-02-17T08:01:26.673609Z  WARN Memory usage is high: 90.26%, we recommend using the prover network
2026-02-17T08:01:54.626976Z  INFO generate GKR circuit: close time.busy=3.19s time.idle=18.0µs
2026-02-17T08:01:56.733139Z  WARN Memory usage is high: 85.11%, we recommend using the prover network
2026-02-17T08:01:59.368949Z  INFO prove GKR circuit: close time.busy=4.74s time.idle=5.58µs
2026-02-17T08:02:06.736656Z  WARN Memory usage is high: 81.00%, we recommend using the prover network
2026-02-17T08:02:16.649523Z  INFO compute jagged values: close time.busy=6.73s time.idle=63.8µs
2026-02-17T08:02:16.771339Z  WARN Memory usage is high: 91.66%, we recommend using the prover network
2026-02-17T08:02:18.697568Z  INFO generate GKR circuit: close time.busy=9.61s time.idle=6.58µs
2026-02-17T08:02:20.245917Z  INFO prove GKR circuit: close time.busy=1.55s time.idle=3.21µs
2026-02-17T08:02:22.834518Z  INFO compute jagged values: close time.busy=146ms time.idle=2.17µs
2026-02-17T08:02:25.881522Z  INFO submitting task of kind RecursionReduce
2026-02-17T08:02:26.787064Z  WARN Memory usage is high: 80.61%, we recommend using the prover network
2026-02-17T08:02:36.370064Z  INFO generate GKR circuit: close time.busy=875ms time.idle=5.83µs
2026-02-17T08:02:36.797118Z  WARN Memory usage is high: 80.30%, we recommend using the prover network
2026-02-17T08:02:37.922417Z  INFO prove GKR circuit: close time.busy=1.55s time.idle=875ns
2026-02-17T08:02:39.717728Z  INFO compute jagged values: close time.busy=142ms time.idle=1.67µs
2026-02-17T08:02:44.620089Z  INFO No neighboring range found, adding proof to tree
2026-02-17T08:02:46.806579Z  WARN Memory usage is high: 83.26%, we recommend using the prover network
2026-02-17T08:02:56.880550Z  WARN Memory usage is high: 89.36%, we recommend using the prover network
2026-02-17T08:03:23.942023Z  INFO generate GKR circuit: close time.busy=1.20s time.idle=7.21µs
2026-02-17T08:03:27.550573Z  INFO generate GKR circuit: close time.busy=354ms time.idle=1.96µs
2026-02-17T08:03:27.582331Z  INFO prove GKR circuit: close time.busy=3.64s time.idle=1.21µs
2026-02-17T08:03:29.314211Z  INFO compute jagged values: close time.busy=148ms time.idle=1.79µs
2026-02-17T08:03:30.968813Z  INFO prove GKR circuit: close time.busy=3.42s time.idle=1.42µs
2026-02-17T08:03:32.851895Z  INFO compute jagged values: close time.busy=139ms time.idle=1.83µs
2026-02-17T08:03:34.805005Z  INFO Setting full range to: Some(ShardRange { timestamp_range: (1, 51147409), initialized_address_range: (0, 120259084568), finalized_address_range: (0, 120259084568), initialized_page_index_range: (0, 0), finalized_page_index_range: (0, 0), deferred_proof_range: (0, 0) })
2026-02-17T08:03:34.805026Z  INFO Sending last core proof to proof queue: Some(RecursionProof { shard_range: ShardRange { timestamp_range: (51147409, 51147409), initialized_address_range: (2025422264, 120259084568), finalized_address_range: (2025032472, 120259084568), initialized_page_index_range: (0, 0), finalized_page_index_range: (0, 0), deferred_proof_range: (0, 0) }, proof: Artifact("artifact_01khn9kbxmfy1sv4wvjcaverzy") })
2026-02-17T08:03:34.805049Z  INFO submitting task of kind RecursionReduce
2026-02-17T08:03:36.914518Z  INFO generate GKR circuit: close time.busy=314ms time.idle=1.42µs
2026-02-17T08:03:38.302344Z  INFO prove GKR circuit: close time.busy=1.39s time.idle=834ns
2026-02-17T08:03:39.951178Z  INFO compute jagged values: close time.busy=140ms time.idle=2.12µs
2026-02-17T08:03:41.991048Z  INFO submitting task of kind ShrinkWrap
2026-02-17T08:03:43.362738Z  INFO generate GKR circuit: close time.busy=81.5ms time.idle=2.12µs
2026-02-17T08:03:43.966161Z  INFO prove GKR circuit: close time.busy=603ms time.idle=1.50µs
2026-02-17T08:03:44.402466Z  INFO compute jagged values: close time.busy=29.7ms time.idle=1.71µs
2026-02-17T08:03:46.045801Z  INFO prove shrink: close time.busy=631ms time.idle=3.42s
2026-02-17T08:04:54.281113Z  INFO generate GKR circuit: close time.busy=444ms time.idle=1.38µs
2026-02-17T08:04:55.836530Z  INFO prove GKR circuit: close time.busy=1.55s time.idle=917ns
2026-02-17T08:04:57.430471Z  INFO compute jagged values: close time.busy=133ms time.idle=1.54µs
2026-02-17T08:05:27.085124Z  INFO prove wrap: close time.busy=27.3s time.idle=73.7s
2026-02-17T08:05:27.356420Z  INFO submitting task of kind Groth16Wrap
2026-02-17T08:05:27.359034Z  INFO [sp1] groth16 circuit artifacts for version v6.0.0-beta.1 do not exist at /Users/varundoshi/.sp1/circuits/groth16/v6.0.0-beta.1. downloading...
  [00:41:08] [########################################################################################################] 5.78 GiB/5.78 GiB (2.40 MiB/s, 0s)[sp1] downloaded https://sp1-circuits.s3-us-east-2.amazonaws.com/v6.0.0-beta.1-groth16.tar.gz to "/Users/varundoshi/.sp1/circuits/groth16/v6.0.0-beta.1"
Setting environment variables took 936.125µs
Reading R1CS took 8.277779209s
Reading proving key took 2.056653958s
Reading witness file took 1.929084ms
Deserializing JSON data took 13.363ms
Generating witness took 302.76575ms
14:16:56 DBG constraint system solver done nbConstraints=15964207 took=2830.982208
14:17:13 DBG prover done acceleration=none backend=groth16 curve=bn254 nbConstraints=15964207 took=16301.605917
Generating proof took 19.138314916s
14:17:13 WRN ignoring uninitialized slice slice name=Vars slice type=[]frontend.Variable
14:17:13 WRN ignoring uninitialized slice slice name=Vars slice type=[]frontend.Variable
14:17:13 WRN ignoring uninitialized slice slice name=Vars slice type=[]frontend.Variable
14:17:13 DBG verifier done backend=groth16 curve=bn254 took=1.078583
2026-02-17T08:47:13.229910Z  INFO prove groth16: close time.busy=711µs time.idle=29.9s
Successfully generated proof!
public values: 0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000220000000000000000000000000000000000000000000000000000000069752f44000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000007a0402f87683064aba80840bebc200840bebc2008401c9c3809407620929b4c0fc48060c4faf378b1e85681c9de1872386f26fc1000080c001a0a47813853bd8456a2f545cffbbd8d6be8292f7e43d6f75cbb61b242ec0a60ab9a0568dd62109b55d483e6407598a69ca62a2e77d25eea4c192962f71eb1ad9c472000000000000
proof: 0x58b7a3c3000000000000000000000000000000000000000000000000000000000000000000410a5637e8b7b8c4b895991b4892efd0ff4da2b5e277d701f2f5c1f23d0c7b00000000000000000000000000000000000000000000000000000000000000002b61148da7216890e530bf16e5c7ae41b4ee8d8d017e5ee86ad6a4f8539980112ed1f1e057d311681d54d451a3f8e2546ccc09c7b313539c0f71c571fd33b9d22301589a7f5fcffcb1577125d6f716642af2dc771932787d66c08c364ea3d09b078a26736c927686cdc4e49c72c84da8f4ba3da7ca34f95f6089adbbf0164c6f28c21155e544209f028516e4837b87b331a230f299b9e27c7095f2e4501ac97a036c41401d03942e1bb35a2331edab31298e0aa583e8ecfbcc0526e2ebe18f16082d5784571efc488a2e39221f5c57a056f526908e453f6883d43b415c7b401b1b741de6cc647c292a9709e730dea5e76b32249e4f33574b62b7ded20f110c30
Successfully verified proof!
Proof generation time: 3052225ms
```