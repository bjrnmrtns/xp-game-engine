**Todo**
- Implement light model, directional light, spot light, point light, ssao?, shadow mapping
- Frustum culling
- Procedural meshes, Box, UVSphere
- Simple two sphere model for physics
- Add some boxes procedurally located to the world
- clipmap interpolate height between different clipmap levels 

**Howto low poly on clipmap terrain**
- for every clipmap item we have a height and two normals
- the vbo contains a provoking vertex for every triangle
- every vbo item indexes into one of the two normal entries
- so every vbo item contains a description which says zero or one
- stil use flat on fragment shader