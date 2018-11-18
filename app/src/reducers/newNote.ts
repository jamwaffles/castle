import {
  NEW_NOTE_CHANGE,
  NEW_NOTE_CREATE,
  NEW_NOTE_CREATE_SUCCESS,
  NewNoteChange,
  NewNoteCreate,
  NewNoteCreateSuccess
} from "../actions";
import { NewNote } from "../types";

export interface NewNoteState {
  note: NewNote;
}

export const defaultState: NewNoteState = { note: { title: "", content: "" } };

export default function auth(
  state = defaultState,
  action = {} as NewNoteChange | NewNoteCreate | NewNoteCreateSuccess
): NewNoteState {
  switch (action.type) {
    case NEW_NOTE_CHANGE:
      return {
        ...state,
        note: action.note
      };

    case NEW_NOTE_CREATE_SUCCESS:
      return defaultState;

    default:
      return state;
  }
}
