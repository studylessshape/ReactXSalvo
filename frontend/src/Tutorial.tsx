import { useState } from "react";
import "virtual:uno.css";

export default function EditProfile() {
  const [isEditing, setIsEditing] = useState(false);
  const [firstName, setFirstName] = useState("Jane");
  const [lastName, setLastName] = useState("Jacobs");

  return (
    <>
      <form
        className="flex flex-col"
        onSubmit={(e) => {
          e.preventDefault();
          setIsEditing((v) => !v);
        }}
      >
        <label>
          First name: {isEditing
            ? (
              <input
                value={firstName}
                onChange={(e) => setFirstName(e.target.value)}
              />
            )
            : <b>{firstName}</b>}
        </label>
        <label>
          Last name: {isEditing
            ? (
              <input
                value={lastName}
                onChange={(e) => setLastName(e.target.value)}
              />
            )
            : <b>{lastName}</b>}
        </label>
        <button type="submit">
          {isEditing ? "Save Profile" : "Edit Profile"}
        </button>
        <p>
          <i>Hello, {lastName} {firstName}!</i>
        </p>
      </form>
    </>
  );
}
