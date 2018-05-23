Rebuild is an attempt at reimplementing Ken Silverman's Build engine in Rust,
with the goal of being modular enough to serve as the base for a modern Blood
source port. The motivation comes from the fact that [BloodGDX][1], the current
source port being recommended by many, is nonfree, and written in Java.

Additionally, existing source ports such as [EDuke32][2] which are built upon
Ken Silverman's codebase are filled with legacy cruft and are subject to the
restrictions of the BUILD license.


[1]: https://blood-wiki.org/index.php/BloodGDX
[2]: http://eduke32.com/
