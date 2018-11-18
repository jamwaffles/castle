import { combineReducers } from "redux";

import newNote, { NewNoteState } from "./newNote";
import notes, { NotesState } from "./notes";

export { NewNoteState, NotesState };

export interface State {
  newNote: NewNoteState;
  notes: NotesState;
}

export interface Reducers {
  newNote: () => NewNoteState;
  notes: () => NotesState;
}

const combined: Reducers = {
  newNote,
  notes
};

export default combineReducers(combined);
