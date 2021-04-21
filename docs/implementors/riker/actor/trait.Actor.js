(function() {var implementors = {};
implementors["communication"] = [{"text":"impl&lt;Req, Res, ClientMsg, P&gt; Actor for <a class=\"struct\" href=\"communication/actor/struct.CommunicationActor.html\" title=\"struct communication::actor::CommunicationActor\">CommunicationActor</a>&lt;Req, Res, ClientMsg, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Req: <a class=\"trait\" href=\"communication/behaviour/protocol/trait.MessageEvent.html\" title=\"trait communication::behaviour::protocol::MessageEvent\">MessageEvent</a> + <a class=\"trait\" href=\"communication/actor/firewall/trait.ToPermissionVariants.html\" title=\"trait communication::actor::firewall::ToPermissionVariants\">ToPermissionVariants</a>&lt;P&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;ClientMsg&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Res: <a class=\"trait\" href=\"communication/behaviour/protocol/trait.MessageEvent.html\" title=\"trait communication::behaviour::protocol::MessageEvent\">MessageEvent</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;ClientMsg: Message,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Message + <a class=\"trait\" href=\"communication/actor/firewall/trait.VariantPermission.html\" title=\"trait communication::actor::firewall::VariantPermission\">VariantPermission</a>,&nbsp;</span>","synthetic":false,"types":["communication::actor::CommunicationActor"]}];
implementors["iota_stronghold"] = [{"text":"impl Actor for <a class=\"struct\" href=\"iota_stronghold/state/client/struct.Client.html\" title=\"struct iota_stronghold::state::client::Client\">Client</a>","synthetic":false,"types":["iota_stronghold::state::client::Client"]},{"text":"impl Actor for <a class=\"struct\" href=\"iota_stronghold/actors/internal/struct.InternalActor.html\" title=\"struct iota_stronghold::actors::internal::InternalActor\">InternalActor</a>&lt;<a class=\"struct\" href=\"iota_stronghold/internals/provider/struct.Provider.html\" title=\"struct iota_stronghold::internals::provider::Provider\">Provider</a>&gt;","synthetic":false,"types":["iota_stronghold::actors::internal::InternalActor"]},{"text":"impl Actor for <a class=\"struct\" href=\"iota_stronghold/state/snapshot/struct.Snapshot.html\" title=\"struct iota_stronghold::state::snapshot::Snapshot\">Snapshot</a>","synthetic":false,"types":["iota_stronghold::state::snapshot::Snapshot"]}];
implementors["stronghold_utils"] = [{"text":"impl&lt;Msg:&nbsp;Message&gt; Actor for <a class=\"struct\" href=\"stronghold_utils/ask/struct.AskActor.html\" title=\"struct stronghold_utils::ask::AskActor\">AskActor</a>&lt;Msg&gt;","synthetic":false,"types":["stronghold_utils::ask::AskActor"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()