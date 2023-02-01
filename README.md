# Engage++

Engage++ is an educational tool built to showcase examples of bas systems design.

It focuses on a learning platform called Aula, where its purpose is to crawl through a given space, and react to every single post with all the emoji that Aula support.

This was inspired by efforts at Coventry University to urge the university to let students be able to opt out of spaces that were solely being used for marketing material to the point where important notifications by lecturers were being drowned out.

---
To use this tool, copy `config.example.json` to `config.json`, and grab your session token from the `x-session-token` header in any authenticated request made to Aula.
Pre-filled are the subdomain that Coventry University operates on, and the workspace that has been the primary cause of issues for this project.

This tool is provided as-is and is currently not guaranteed to be stable, nor properly optimized. As such, [caveat emptor](https://en.wikipedia.org/wiki/Caveat_emptor), bugs may occur.

The author(s) of this tool do(es) not encourage the use of this tool outside an educational environment.

---
## Contributing
The current to-do list is as follows:

- Improve performance (parallelization, checking if batching is possible, etc)
- Add support for custom emoji on Aula
- Add documentation
- Add tests (might be impossible)
- Filtering to allow only educator posts to be hit, or posts by certain individuals
