import { NEW_NOTE_CREATE_SUCCESS, NewNoteCreateSuccess } from "../actions";
import { AnyNote } from "../types";

export interface NotesState {
  notes: AnyNote[];
}

export const defaultState: NotesState = { notes: [] };

export default function notes(
  state = defaultState,
  action = {} as NewNoteCreateSuccess
): NotesState {
  switch (action.type) {
    case NEW_NOTE_CREATE_SUCCESS:
      return {
        ...state,
        notes: state.notes.concat([action.note])
      };

    default:
      return state;
  }
}
