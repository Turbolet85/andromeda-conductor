# layouts extract

## No domain coverage
The runs.db index chunk (persistence layer, schema design, bound-parameter writes) has no layout/wireframe/component-placement concerns; layout templates (§Primary Surfaces, §Wireframe, §Component placement) address surface rendering, and this chunk is exclusively a synchronous database seam with no UI surface involvement.
