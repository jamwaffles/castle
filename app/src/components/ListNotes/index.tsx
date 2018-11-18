import * as React from "react";
import { connect } from "react-redux";
import { Note } from "../../types";

export interface ListNotesProps {
  notes: Note[];
}

export default class ListNotes extends React.PureComponent<
  ListNotesProps,
  any
> {
  render() {
    return (
      <div>
        <div>A list of notes</div>

        <ul>
          {this.props.notes.map((note: Note) => (
            <li key={note.id}>
              <strong>{note.title}</strong> {note.content}
            </li>
          ))}
        </ul>
      </div>
    );
  }
}
