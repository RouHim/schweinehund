/// <reference path="../pb_data/types.d.ts" />

onAfterBootstrap((e) => {
  $app.onBeforeServe().add((e) => {
    e.router.GET("/partials/:name", (c) => {
      const name = c.pathParam("name")
      
      if (!/^[a-z0-9-]+$/i.test(name)) {
        return c.json(400, { "error": "Invalid partial name" })
      }
      
      const filePath = `${__hooks}/../pb_public/partials/${name}.html`
      
      try {
        const content = $os.readFile(filePath)
        const html = String.fromArrayBuffer(content)
        return c.html(200, html)
      } catch (err) {
        console.error("Partial not found:", name, err)
        return c.json(404, { "error": "Partial not found" })
      }
    })
    
    console.log("Partial routes registered: /partials/:name")
  })
})
