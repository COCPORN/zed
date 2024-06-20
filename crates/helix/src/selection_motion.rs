use crate::{
    normal::{mark::create_mark, repeat},
    state::Mode,
    Helix,
};
use editor::{scroll::Autoscroll, Bias};
use gpui::{actions, Action, ViewContext};
use language::SelectionGoal;
use workspace::Workspace;

actions!(
    helix,
    [
        MoveNextWordStart,
        MovePrevWordStart,
        MoveNextWordEnd,
        MoveNextLongWordStart
    ]
);

pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(move_next_word_start);
    workspace.register_action(move_prev_word_start);
}

fn move_next_word_start(
    _: &mut Workspace,
    action: &MoveNextWordStart,
    cx: &mut ViewContext<Workspace>,
) {
}

fn move_prev_word_start(
    _: &mut Workspace,
    action: &MovePrevWordStart,
    cx: &mut ViewContext<Workspace>,
) {
}
