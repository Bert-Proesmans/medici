(function() {var implementors = {};
implementors["game_system"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"game_system/prototype/enum.ProtoItem.html\" title=\"enum game_system::prototype::ProtoItem\">ProtoItem</a>",synthetic:false,types:["game_system::prototype::definition::ProtoItem"]},{text:"impl&lt;X:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, CTS:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"game_system/state_machine/machine/struct.Machine.html\" title=\"struct game_system::state_machine::machine::Machine\">Machine</a>&lt;X, CTS&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;X: <a class=\"trait\" href=\"game_system/re_export/marker/trait.TopLevel.html\" title=\"trait game_system::re_export::marker::TopLevel\">TopLevel</a> + <a class=\"trait\" href=\"game_system/prelude/function/trait.State.html\" title=\"trait game_system::prelude::function::State\">State</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;CTS: <a class=\"trait\" href=\"medici_core/ctstack/trait.CTStack.html\" title=\"trait medici_core::ctstack::CTStack\">CTStack</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;X::<a class=\"type\" href=\"game_system/prelude/function/trait.State.html#associatedtype.Transaction\" title=\"type game_system::prelude::function::State::Transaction\">Transaction</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>",synthetic:false,types:["game_system::state_machine::machine::Machine"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"game_system/state_machine/state/leaf/triggerable/struct.Start.html\" title=\"struct game_system::state_machine::state::leaf::triggerable::Start\">Start</a>",synthetic:false,types:["game_system::state_machine::state::leaf::triggerable::Start"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"game_system/state_machine/state/leaf/triggerable/struct.Input.html\" title=\"struct game_system::state_machine::state::leaf::triggerable::Input\">Input</a>",synthetic:false,types:["game_system::state_machine::state::leaf::triggerable::Input"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"game_system/state_machine/state/leaf/triggerable/struct.EndTurn.html\" title=\"struct game_system::state_machine::state::leaf::triggerable::EndTurn\">EndTurn</a>",synthetic:false,types:["game_system::state_machine::state::leaf::triggerable::EndTurn"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"game_system/state_machine/state/leaf/triggerable/struct.PlayCard.html\" title=\"struct game_system::state_machine::state::leaf::triggerable::PlayCard\">PlayCard</a>",synthetic:false,types:["game_system::state_machine::state::leaf::triggerable::PlayCard"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"game_system/state_machine/state/leaf/triggerable/struct.Attack.html\" title=\"struct game_system::state_machine::state::leaf::triggerable::Attack\">Attack</a>",synthetic:false,types:["game_system::state_machine::state::leaf::triggerable::Attack"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"game_system/state_machine/state/leaf/triggerable/struct.Damage.html\" title=\"struct game_system::state_machine::state::leaf::triggerable::Damage\">Damage</a>",synthetic:false,types:["game_system::state_machine::state::leaf::triggerable::Damage"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"game_system/state_machine/state/leaf/triggerable/enum.TriggerItem.html\" title=\"enum game_system::state_machine::state::leaf::triggerable::TriggerItem\">TriggerItem</a>",synthetic:false,types:["game_system::state_machine::state::leaf::triggerable::TriggerItem"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"game_system/state_machine/transaction/enum.TransactionItem.html\" title=\"enum game_system::state_machine::transaction::TransactionItem\">TransactionItem</a>",synthetic:false,types:["game_system::state_machine::transaction::TransactionItem"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"game_system/tag/enum.EntityTags.html\" title=\"enum game_system::tag::EntityTags\">EntityTags</a>",synthetic:false,types:["game_system::tag::EntityTags"]},];
implementors["medici_core"] = [{text:"impl&lt;S:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, ETM:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, ETR:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/card/struct.CardStruct.html\" title=\"struct medici_core::prefab::card::CardStruct\">CardStruct</a>&lt;S, ETM, ETR&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;ETM: <a class=\"trait\" href=\"medici_core/marker/trait.TimingEnumerator.html\" title=\"trait medici_core::marker::TimingEnumerator\">TimingEnumerator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;ETR: <a class=\"trait\" href=\"medici_core/marker/trait.TriggerEnumerator.html\" title=\"trait medici_core::marker::TriggerEnumerator\">TriggerEnumerator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,&nbsp;</span>",synthetic:false,types:["medici_core::prefab::card::CardStruct"]},{text:"impl&lt;S:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, P:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/entity/struct.EntityStruct.html\" title=\"struct medici_core::prefab::entity::EntityStruct\">EntityStruct</a>&lt;S, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"medici_core/marker/trait.ProtoEnumerator.html\" title=\"trait medici_core::marker::ProtoEnumerator\">ProtoEnumerator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,&nbsp;</span>",synthetic:false,types:["medici_core::prefab::entity::EntityStruct"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"medici_core/prefab/prototype/enum.ProtoItem.html\" title=\"enum medici_core::prefab::prototype::ProtoItem\">ProtoItem</a>",synthetic:false,types:["medici_core::prefab::prototype::ProtoItem"]},{text:"impl&lt;W:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"medici_core/marker/trait.Waitable.html\" title=\"trait medici_core::marker::Waitable\">Waitable</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/state/struct.Wait.html\" title=\"struct medici_core::prefab::state::Wait\">Wait</a>&lt;W&gt;",synthetic:false,types:["medici_core::prefab::state::Wait"]},{text:"impl&lt;A:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"medici_core/marker/trait.Actionable.html\" title=\"trait medici_core::marker::Actionable\">Actionable</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/state/struct.Action.html\" title=\"struct medici_core::prefab::state::Action\">Action</a>&lt;A&gt;",synthetic:false,types:["medici_core::prefab::state::Action"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/state/struct.Finished.html\" title=\"struct medici_core::prefab::state::Finished\">Finished</a>",synthetic:false,types:["medici_core::prefab::state::Finished"]},{text:"impl&lt;A:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"medici_core/marker/trait.Actionable.html\" title=\"trait medici_core::marker::Actionable\">Actionable</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/state/struct.Effect.html\" title=\"struct medici_core::prefab::state::Effect\">Effect</a>&lt;A&gt;",synthetic:false,types:["medici_core::prefab::state::Effect"]},{text:"impl&lt;TR:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"medici_core/marker/trait.Triggerable.html\" title=\"trait medici_core::marker::Triggerable\">Triggerable</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/state/struct.RecurseEffect.html\" title=\"struct medici_core::prefab::state::RecurseEffect\">RecurseEffect</a>&lt;TR&gt;",synthetic:false,types:["medici_core::prefab::state::RecurseEffect"]},{text:"impl&lt;TM:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"medici_core/marker/trait.Timing.html\" title=\"trait medici_core::marker::Timing\">Timing</a>, TR:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"medici_core/marker/trait.Triggerable.html\" title=\"trait medici_core::marker::Triggerable\">Triggerable</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/state/struct.DeathEffect.html\" title=\"struct medici_core::prefab::state::DeathEffect\">DeathEffect</a>&lt;TM, TR&gt;",synthetic:false,types:["medici_core::prefab::state::DeathEffect"]},{text:"impl&lt;TM:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"medici_core/marker/trait.Timing.html\" title=\"trait medici_core::marker::Timing\">Timing</a>, TR:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"medici_core/marker/trait.Triggerable.html\" title=\"trait medici_core::marker::Triggerable\">Triggerable</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/state/struct.Trigger.html\" title=\"struct medici_core::prefab::state::Trigger\">Trigger</a>&lt;TM, TR&gt;",synthetic:false,types:["medici_core::prefab::state::Trigger"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/timing/struct.Pre.html\" title=\"struct medici_core::prefab::timing::Pre\">Pre</a>",synthetic:false,types:["medici_core::prefab::timing::Pre"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/timing/struct.Peri.html\" title=\"struct medici_core::prefab::timing::Peri\">Peri</a>",synthetic:false,types:["medici_core::prefab::timing::Peri"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/timing/struct.Post.html\" title=\"struct medici_core::prefab::timing::Post\">Post</a>",synthetic:false,types:["medici_core::prefab::timing::Post"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"medici_core/prefab/timing/enum.TimingItem.html\" title=\"enum medici_core::prefab::timing::TimingItem\">TimingItem</a>",synthetic:false,types:["medici_core::prefab::timing::TimingItem"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/transaction/struct.Epsilon.html\" title=\"struct medici_core::prefab::transaction::Epsilon\">Epsilon</a>",synthetic:false,types:["medici_core::prefab::transaction::Epsilon"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/trigger/struct.GameStart.html\" title=\"struct medici_core::prefab::trigger::GameStart\">GameStart</a>",synthetic:false,types:["medici_core::prefab::trigger::GameStart"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/trigger/struct.GameEnd.html\" title=\"struct medici_core::prefab::trigger::GameEnd\">GameEnd</a>",synthetic:false,types:["medici_core::prefab::trigger::GameEnd"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/trigger/struct.TurnStart.html\" title=\"struct medici_core::prefab::trigger::TurnStart\">TurnStart</a>",synthetic:false,types:["medici_core::prefab::trigger::TurnStart"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/prefab/trigger/struct.TurnEnd.html\" title=\"struct medici_core::prefab::trigger::TurnEnd\">TurnEnd</a>",synthetic:false,types:["medici_core::prefab::trigger::TurnEnd"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"medici_core/prefab/trigger/enum.TriggerItem.html\" title=\"enum medici_core::prefab::trigger::TriggerItem\">TriggerItem</a>",synthetic:false,types:["medici_core::prefab::trigger::TriggerItem"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"medici_core/error/custom_type/enum.TriggerFail.html\" title=\"enum medici_core::error::custom_type::TriggerFail\">TriggerFail</a>",synthetic:false,types:["medici_core::error::custom_type::TriggerFail"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"medici_core/error/enum.ErrorKind.html\" title=\"enum medici_core::error::ErrorKind\">ErrorKind</a>",synthetic:false,types:["medici_core::error::ErrorKind"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/function/struct.CardId.html\" title=\"struct medici_core::function::CardId\">CardId</a>",synthetic:false,types:["medici_core::function::CardId"]},{text:"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/service/entity/struct.EntityService.html\" title=\"struct medici_core::service::entity::EntityService\">EntityService</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"medici_core/function/trait.Entity.html\" title=\"trait medici_core::function::Entity\">Entity</a> + <a class=\"trait\" href=\"medici_core/function/trait.EntityBuilder.html\" title=\"trait medici_core::function::EntityBuilder\">EntityBuilder</a>&lt;E&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E::<a class=\"type\" href=\"medici_core/function/trait.Identifiable.html#associatedtype.ID\" title=\"type medici_core::function::Identifiable::ID\">ID</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a>,&nbsp;</span>",synthetic:false,types:["medici_core::service::entity::EntityService"]},{text:"impl&lt;ETM:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, ETR:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/service/trigger/struct.TriggerService.html\" title=\"struct medici_core::service::trigger::TriggerService\">TriggerService</a>&lt;ETM, ETR&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ETM: <a class=\"trait\" href=\"medici_core/marker/trait.TimingEnumerator.html\" title=\"trait medici_core::marker::TimingEnumerator\">TimingEnumerator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;ETR: <a class=\"trait\" href=\"medici_core/marker/trait.TriggerEnumerator.html\" title=\"trait medici_core::marker::TriggerEnumerator\">TriggerEnumerator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,&nbsp;</span>",synthetic:false,types:["medici_core::service::trigger::TriggerService"]},{text:"impl&lt;C:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/storage/card/struct.CardStorage.html\" title=\"struct medici_core::storage::card::CardStorage\">CardStorage</a>&lt;C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"medici_core/function/trait.Card.html\" title=\"trait medici_core::function::Card\">Card</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;C as <a class=\"trait\" href=\"medici_core/function/trait.Identifiable.html\" title=\"trait medici_core::function::Identifiable\">Identifiable</a>&gt;::<a class=\"type\" href=\"medici_core/function/trait.Identifiable.html#associatedtype.ID\" title=\"type medici_core::function::Identifiable::ID\">ID</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;C::<a class=\"type\" href=\"medici_core/function/trait.Card.html#associatedtype.TimingEnum\" title=\"type medici_core::function::Card::TimingEnum\">TimingEnum</a>: <a class=\"trait\" href=\"medici_core/marker/trait.TimingEnumerator.html\" title=\"trait medici_core::marker::TimingEnumerator\">TimingEnumerator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;C::<a class=\"type\" href=\"medici_core/function/trait.Card.html#associatedtype.TriggerEnum\" title=\"type medici_core::function::Card::TriggerEnum\">TriggerEnum</a>: <a class=\"trait\" href=\"medici_core/marker/trait.TriggerEnumerator.html\" title=\"trait medici_core::marker::TriggerEnumerator\">TriggerEnumerator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;C::<a class=\"type\" href=\"medici_core/function/trait.Identifiable.html#associatedtype.ID\" title=\"type medici_core::function::Identifiable::ID\">ID</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>",synthetic:false,types:["medici_core::storage::card::CardStorage"]},{text:"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/storage/entity/struct.EntityStorage.html\" title=\"struct medici_core::storage::entity::EntityStorage\">EntityStorage</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"medici_core/function/trait.Entity.html\" title=\"trait medici_core::function::Entity\">Entity</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E::<a class=\"type\" href=\"medici_core/function/trait.Identifiable.html#associatedtype.ID\" title=\"type medici_core::function::Identifiable::ID\">ID</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt;,&nbsp;</span>",synthetic:false,types:["medici_core::storage::entity::EntityStorage"]},{text:"impl&lt;TTC:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/storage/transaction/struct.TransactionStorage.html\" title=\"struct medici_core::storage::transaction::TransactionStorage\">TransactionStorage</a>&lt;TTC&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TTC: <a class=\"trait\" href=\"medici_core/marker/trait.TransactionContainer.html\" title=\"trait medici_core::marker::TransactionContainer\">TransactionContainer</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>",synthetic:false,types:["medici_core::storage::transaction::TransactionStorage"]},{text:"impl&lt;ETM:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, ETR:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/storage/trigger/struct.UnsafeTrigger.html\" title=\"struct medici_core::storage::trigger::UnsafeTrigger\">UnsafeTrigger</a>&lt;ETM, ETR&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ETM: <a class=\"trait\" href=\"medici_core/marker/trait.TimingEnumerator.html\" title=\"trait medici_core::marker::TimingEnumerator\">TimingEnumerator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;ETR: <a class=\"trait\" href=\"medici_core/marker/trait.TriggerEnumerator.html\" title=\"trait medici_core::marker::TriggerEnumerator\">TriggerEnumerator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,&nbsp;</span>",synthetic:false,types:["medici_core::storage::trigger::UnsafeTrigger"]},{text:"impl&lt;ETM:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, ETR:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"medici_core/storage/trigger/struct.TriggerStorage.html\" title=\"struct medici_core::storage::trigger::TriggerStorage\">TriggerStorage</a>&lt;ETM, ETR&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ETM: <a class=\"trait\" href=\"medici_core/marker/trait.TimingEnumerator.html\" title=\"trait medici_core::marker::TimingEnumerator\">TimingEnumerator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;ETR: <a class=\"trait\" href=\"medici_core/marker/trait.TriggerEnumerator.html\" title=\"trait medici_core::marker::TriggerEnumerator\">TriggerEnumerator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,&nbsp;</span>",synthetic:false,types:["medici_core::storage::trigger::TriggerStorage"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
