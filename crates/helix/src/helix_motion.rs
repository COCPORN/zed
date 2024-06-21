use crate::Helix;
use editor::{
    display_map::DisplaySnapshot,
    movement::{find_boundary, FindRange},
    scroll::Autoscroll,
    DisplayPoint,
};
use gpui::{actions, ViewContext};
use language::{char_kind, SelectionGoal};
use workspace::Workspace;

actions!(
    helix,
    [
        MoveNextWordStart,
        MovePrevWordStart,
        MoveNextWordEnd,
        MoveNextLongWordStart,
        ExtendLineBelow,
    ]
);

pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(move_next_word_start);
    workspace.register_action(move_prev_word_start);
}

fn move_next_word_start(_: &mut Workspace, _: &MoveNextWordStart, cx: &mut ViewContext<Workspace>) {
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

    // Use this to see if the character kind has changed, but keep
    // grabbing whitespace
    let mut flipped = false;
    find_boundary(map, point, FindRange::MultiLine, |left, right| {
        flipped = char_kind(&scope, left) != char_kind(&scope, right) && !right.is_whitespace();
        if right.is_whitespace() && right != '\n' {
            false
        } else if left == '\n' {
            true
        } else {
            flipped
        }
    })
}

fn move_prev_word_start(_: &mut Workspace, _: &MovePrevWordStart, cx: &mut ViewContext<Workspace>) {
    Helix::update(cx, |hx, cx| {
        hx.update_active_editor(cx, |_, editor, cx| {
            editor.cancel(&editor::actions::Cancel, cx);
            editor.select_to_previous_word_start(&editor::actions::SelectToPreviousWordStart, cx);
        });
    });
}
