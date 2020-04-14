export type IsoDate = string;
export type Uuid = string;

// export interface ChecklistItem {
//   content: string;
//   created_at: IsoDate;
//   checked: boolean;
// }

// export type NoteContent = string | ChecklistItem;

// export interface NewNote {
//   title?: string;
//   content?: NoteContent;
// }

// export interface Note {
//   id: Uuid;
//   title: string;
//   content: NoteContent;
//   created_at: IsoDate;
// }

// export type AnyNote = Note | NewNote;




export interface ChecklistItem {
  content: string;
  created_at: IsoDate;
  checked: boolean;
  position: number;
}

export interface NoteCommon {
  id: Uuid;
  title: string;
  created_at: IsoDate;
  updated_at: IsoDate;
  saved: boolean;
  note_type: NoteType;
}

export interface TextNote extends NoteCommon {
  content: string;
  note_type: 'Text';
}

export interface ChecklistNote extends NoteCommon {
  items: ChecklistItem[];
  note_type: 'Checklist';
}

export type AnyNote = TextNote | ChecklistNote;
export type NoteType = 'Text' | 'Checklist';