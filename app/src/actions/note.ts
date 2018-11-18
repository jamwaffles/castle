import { AnyAction } from "redux";
import { NewNote, Note } from "../types";
import { v4 } from "uuid";

export const NEW_NOTE_CHANGE = "NEW_NOTE_CHANGE";
export const NEW_NOTE_CREATE = "NEW_NOTE_CREATE";
export const NEW_NOTE_CREATE_SUCCESS = "NEW_NOTE_CREATE_SUCCESS";
export const NEW_NOTE_CREATE_FAILED = "NEW_NOTE_CREATE_FAILED";

export interface NewNoteChange {
  type: "NEW_NOTE_CHANGE";
  note: NewNote;
}

export interface NewNoteCreate {
  type: "NEW_NOTE_CREATE";
  note: NewNote;
}

export interface NewNoteCreateSuccess {
  type: "NEW_NOTE_CREATE_SUCCESS";
  note: Note;
}

export interface NewNoteCreateFailed {
  type: "NEW_NOTE_CREATE_FAILED";
  note: Note;
}

export function changeNewNote(note: NewNote): NewNoteChange {
  return {
    type: NEW_NOTE_CHANGE,
    note
  };
}

export function createNote(note: NewNote): NewNoteCreateSuccess {
  return {
    type: NEW_NOTE_CREATE_SUCCESS,
    note: {
      title: note.title || "",
      content: note.content || "",
      created_at: new Date().toISOString(),
      id: v4()
    }
  };
}
