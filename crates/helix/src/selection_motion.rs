use crate::{
    normal::{mark::create_mark, repeat},
    state::Mode,
    Helix,
};
use editor::{
    actions,
    display_map::DisplaySnapshot,
    movement::{find_boundary, find_boundary_exclusive, find_boundary_point, FindRange},
    scroll::Autoscroll,
    Bias, DisplayPoint, Editor,
};
use gpui::{actions, Action, ViewContext};
use language::{char_kind, SelectionGoal};
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
    Helix::update(cx, |hx, cx| {
        hx.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| s.try_cancel());
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_heads_with(|map, head, _| {
                    (select_to_next_word_start(map, head), SelectionGoal::None)
                });
            });
        });
    });
}

fn select_to_next_word_start(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let raw_point = point.to_point(map);
    let scope = map.buffer_snapshot.language_scope_at(raw_point);

    // let non_whitespace_point = find_boundary_point(
    //     map,
    //     point,
    //     FindRange::MultiLine,
    //     |left, right| !left.is_whitespace() || right.is_whitespace(),
    //     true,
    // );

    let mut flipped = false;
    find_boundary(map, point, FindRange::MultiLine, |left, right| {
        flipped = char_kind(&scope, left) != char_kind(&scope, right);
        if right.is_whitespace() && right != '\n' {
            false
        } else {
            flipped
        }
        // (char_kind(&scope, left) != char_kind(&scope, right)) && !left.is_whitespace()
        //     || right == '\n'
    })
}

fn move_prev_word_start(
    _: &mut Workspace,
    action: &MovePrevWordStart,
    cx: &mut ViewContext<Workspace>,
) {
    Helix::update(cx, |hx, cx| {
        hx.update_active_editor(cx, |_, editor, cx| {
            editor.cancel(&editor::actions::Cancel, cx);
            editor.select_to_previous_word_start(&editor::actions::SelectToPreviousWordStart, cx);
        });
    });
}
