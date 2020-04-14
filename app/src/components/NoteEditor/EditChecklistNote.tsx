import * as React from "react";
import {
  AnyNote,
  defaultNote,
  ChecklistNote,
  ChecklistItem
} from "../../types";
import { Option } from "funfix";

export interface EditChecklistNoteProps {
  note: ChecklistNote;
}

export default class EditChecklistNote extends React.PureComponent<
  EditChecklistNoteProps,
  any
> {
  render() {
    return <div className="edit-checklist-note">Edit checklist note</div>;
  }
}
