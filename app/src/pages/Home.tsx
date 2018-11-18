import * as React from "react";
import { ActionCreator } from "redux";
import { connect } from "react-redux";

import {
  changeNewNote,
  createNote,
  NewNoteChange,
  NewNoteCreateSuccess
} from "../actions";
import { State, NewNoteState, NotesState } from "../reducers";

import NoteEditor from "../components/NoteEditor";
import ListNotes from "../components/ListNotes";

export interface HomeProps {
  newNote: NewNoteState;
  notes: NotesState;
  changeNewNote: ActionCreator<NewNoteChange>;
  createNote: ActionCreator<NewNoteCreateSuccess>;
}

class Home extends React.PureComponent<HomeProps, any> {
  handleDone = () => {
    this.props.createNote(this.props.newNote.note);

    return {};
  };

  render() {
    return (
      <div>
        <header>
          <p>
            <strong>Create a note</strong>
          </p>

          <NoteEditor
            note={this.props.newNote.note}
            onChange={this.props.changeNewNote}
            onDone={this.handleDone}
          />
        </header>

        <ListNotes notes={this.props.notes.notes} />
      </div>
    );
  }
}

export default connect(
  (s: State) => ({ newNote: s.newNote, notes: s.notes }),
  { changeNewNote, createNote }
)(Home);
