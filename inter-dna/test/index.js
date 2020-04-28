/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require('path')

const { Orchestrator, Config, combine, singleConductor, localOnly, tapeExecutor } = require('@holochain/tryorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/inter-dna.dna.json")
const dna = Config.dna(dnaPath, 'InterDNA')

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require('tape')),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly,
  ),
  //   waiter: {
  //   hardTimeout: 100000,
  //   strict: true,
  // }
});

const conductorConfig = Config.gen(
  {
    InterDNA: dna,
  },
  {
    logger: {
      type: 'debug',
      state_dump: false,
      // rules: {
      //     rules: [{ exclude: true, pattern: ".*" }]
      // }
    },
    network: {
      type: 'sim2h',
      sim2h_url: 'ws://localhost:9000'
    }
  }
)

const sample_dna_address = "QmZ6mav8UBRzA5YzApoVRdUWQGCw4wgxBvEkkYN1sQXXkH"
const sample_target_dna_address = "QmXuPFimMCoYQrXqX9vr1vve8JtpQ7smfkw1LugqEhyWTr"
const sample_source_address = "14f5e1afcbfb2a7d617ddb3423d742b3959eb36100e3efdc481c1966b4d06858"
const sample_target_address = "62ccd5f507d61e28fe590a6487e120d9bf87bf7d61a447c4ccddbc447382873e"

orchestrator.registerScenario("create and get link outgoing & incoming", async (s, t) => {
  const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig}, true)

  const result = await alice.call("InterDNA", "inter_dna", "create_link", 
    {source: {dna_address: sample_dna_address, entry_address: sample_source_address}, 
      target: {dna_address: sample_target_dna_address, entry_address: sample_target_address}})
  t.deepEqual(result.hasOwnProperty("Ok"), true)
  await s.consistency()

  const outgoing = await bob.call("InterDNA", "inter_dna", "get_outgoing", {source: {dna_address: sample_dna_address, entry_address: sample_source_address}, page: 0, count: 1})
  t.deepEqual(outgoing.hasOwnProperty("Ok"), true)
  t.deepEqual(outgoing.Ok.length, 1)

  const incoming = await bob.call("InterDNA", "inter_dna", "get_incoming", {target: {dna_address: sample_target_dna_address, entry_address: sample_target_address}, page: 0, count: 1})
  t.deepEqual(incoming.hasOwnProperty("Ok"), true)
  t.deepEqual(incoming.Ok.length, 1)
})

orchestrator.registerScenario("create and remove link", async (s, t) => {
  const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig}, true)

  const result = await alice.call("InterDNA", "inter_dna", "create_link", 
    {source: {dna_address: sample_dna_address, entry_address: sample_source_address}, 
      target: {dna_address: sample_target_dna_address, entry_address: sample_target_address}})
  t.deepEqual(result.hasOwnProperty("Ok"), true)
  await s.consistency()

  const outgoing = await bob.call("InterDNA", "inter_dna", "get_outgoing", {source: {dna_address: sample_dna_address, entry_address: sample_source_address}, page: 0, count: 1})
  t.deepEqual(outgoing.hasOwnProperty("Ok"), true)
  t.deepEqual(outgoing.Ok.length, 1)

  const delete_link = await alice.call("InterDNA", "inter_dna", "remove_link", 
    {source: {dna_address: sample_dna_address, entry_address: sample_source_address}, 
      target: {dna_address: sample_target_dna_address, entry_address: sample_target_address}})
  t.deepEqual(delete_link.hasOwnProperty("Ok"), true)
  await s.consistency()

  const outgoing_deleted = await bob.call("InterDNA", "inter_dna", "get_outgoing", {source: {dna_address: sample_dna_address, entry_address: sample_source_address}, page: 0, count: 1})
  t.deepEqual(outgoing_deleted.hasOwnProperty("Ok"), true)
  t.deepEqual(outgoing_deleted.Ok.length, 0)

  const incoming = await bob.call("InterDNA", "inter_dna", "get_incoming", {target: {dna_address: sample_target_dna_address, entry_address: sample_target_address}, page: 0, count: 1})
  t.deepEqual(incoming.hasOwnProperty("Ok"), true)
  t.deepEqual(incoming.Ok.length, 0)
})

orchestrator.registerScenario("create and remove link w/ validation failure", async (s, t) => {
  const {alice, bob, james} = await s.players({alice: conductorConfig, bob: conductorConfig, james: conductorConfig}, true)

  const result = await alice.call("InterDNA", "inter_dna", "create_link", 
    {source: {dna_address: sample_dna_address, entry_address: sample_source_address}, 
      target: {dna_address: sample_target_dna_address, entry_address: sample_target_address}})
  t.deepEqual(result.hasOwnProperty("Ok"), true)
  await s.consistency()

  const delete_link = await james.call("InterDNA", "inter_dna", "remove_link", 
    {source: {dna_address: sample_dna_address, entry_address: sample_source_address}, 
      target: {dna_address: sample_target_dna_address, entry_address: sample_target_address}})
  t.deepEqual(delete_link.hasOwnProperty("Err"), true)
  await s.consistency()

  const outgoing_deleted = await bob.call("InterDNA", "inter_dna", "get_outgoing", {source: {dna_address: sample_dna_address, entry_address: sample_source_address}, page: 0, count: 1})
  t.deepEqual(outgoing_deleted.hasOwnProperty("Ok"), true)
  t.deepEqual(outgoing_deleted.Ok.length, 1)

  const incoming = await bob.call("InterDNA", "inter_dna", "get_incoming", {target: {dna_address: sample_target_dna_address, entry_address: sample_target_address}, page: 0, count: 1})
  t.deepEqual(incoming.hasOwnProperty("Ok"), true)
  t.deepEqual(incoming.Ok.length, 1)
})

orchestrator.run()
