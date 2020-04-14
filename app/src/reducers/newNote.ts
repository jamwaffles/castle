import {
  NEW_NOTE_CHANGE,
  NEW_NOTE_CREATE_SUCCESS,
  NewNoteChange,
  NewNoteCreateSuccess
} from "../actions";
import { AnyNote } from "../types";
import { Option, None, Some } from 'funfix';

export interface NewNoteState {
  note: Option<AnyNote>;
}

export const defaultState: NewNoteState = { note: None };

export default function auth(
  state = defaultState,
  action: NewNoteChange | NewNoteCreateSuccess = {} as any
): NewNoteState {
  switch (action.type) {
    case NEW_NOTE_CHANGE:
      return {
        ...state,
        note: Some(action.note)
      };

    case NEW_NOTE_CREATE_SUCCESS:
      return defaultState;

    default:
      return state;
  }
}
