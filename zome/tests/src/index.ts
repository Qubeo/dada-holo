import {
  Orchestrator,
  Config,
  InstallAgentsHapps,
  TransportConfigType,
  Player,
} from "@holochain/tryorama";
import path from "path";

const conductorConfig = Config.gen({});

// Construct proper paths for your DNAs
const dnaPath = path.join(__dirname, "../../workdir/dna/sample.dna");

const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(), ms));

const orchestrator = new Orchestrator();

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [dnaPath],
  ],
    // agent 1
  [
    // happ 0
    [dnaPath],
  ],
];

// orchestrator.registerScenario(
//   "create and get a calendar event",
//   async (s, t) => {
//     const [player]: Player[] = await s.players([conductorConfig]);
//     const [[alice_happ], [bob_happ]] = await player.installAgentsHapps(
//       installation
//     );

//     const alice_calendar = alice_happ.cells[0];
//     const bob_calendar = bob_happ.cells[0];

//     let calendarEvent = await alice_calendar.call(
//       "tragedy_of_commons",
//       "create_calendar_event",
//       {
//         title: "Event 1",
//         start_time: [Math.floor(Date.now() / 1000), 0],
//         end_time: [Math.floor(Date.now() / 1000) + 1000, 0],
//         location: { Custom: "hiii" },
//         invitees: [],
//       }
//     );
//     t.ok(calendarEvent);

//     await sleep(10);

//     let calendarEvents = await alice_calendar.call(
//       "tragedy_of_commons",
//       "get_all_calendar_events",
//       null
//     );
//     t.equal(calendarEvents.length, 1);

//   }
// );

orchestrator.registerScenario(
  "create posts p2p and retrieve them",
  async (s, t) => {
    const ZOME_NAME = "tragedy_of_commons";
    const [alice, bob, clare] = await s.players([conductorConfig, conductorConfig, conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_common]] = await alice.installAgentsHapps(installation);
    const [[bob_common]] = await bob.installAgentsHapps(installation);
    //const [[clare_common]] = await clare.installAgentsHapps(installation);

    await s.shareAllNodes([alice, bob])

    // let capGrant = await alice_common.cells[0].call(
    //   ZOME_NAME,
    //   "create_cap_access",
    //   {
    //     function: "receive_p2p_message",
    //     agent: bob_common.agent,
    //   }
    // );
    // console.log(capGrant);
    // t.ok(capGrant);
    
    // let capGrantBob = await bob_common.cells[0].call(
    //   ZOME_NAME,
    //   "create_cap_access",
    //   {
    //     function: "receive_p2p_message",
    //     agent: alice_common.agent,
    //   }
    // );
    // console.log(capGrantBob);
    // t.ok(capGrantBob);

    //Alice starts a new game (session) with bob and herself
    let session_header_hash = await alice_common.cells[0].call(
      ZOME_NAME,
      "start_new_session",
      [bob_common.agent, alice_common.agent]
    );
    console.log(session_header_hash);
    t.ok(session_header_hash);
    
  }
);

orchestrator.run();
