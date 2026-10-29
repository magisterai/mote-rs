# List continuation reference

Covers named and unnamed continuation, an explicit restart, later starts on an
existing sequence, and definition changes. A named sequence continues by default.
An unnamed sequence restarts by default; restart=false continues at its base level.
A restart replaces the instance used for subsequent continuation. A later start
is ignored while continuing. Format and geometry are part of definition identity;
levels share a named instance but distinguish unnamed instances.

The model and every package part are compared. Tests independently assert the
instance sequence and start overrides. Word review remains unreviewed.
