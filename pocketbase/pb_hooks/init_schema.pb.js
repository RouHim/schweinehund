/// <reference path="../pb_data/types.d.ts" />

onBootstrap((e) => {
  e.next()
  const collections = $app.findAllCollections()
  const existingNames = collections.map(c => c.name)
  
  // Create zones collection if it doesn't exist
  if (!existingNames.includes('zones')) {
    const zones = new Collection({
      name: 'zones',
      type: 'base',
      listRule: '',
      viewRule: '',
      createRule: '',
      updateRule: '',
      deleteRule: '',
      fields: [
        {
          name: 'name',
          type: 'text',
          required: true,
          max: 100,
        },
        {
          name: 'emoji',
          type: 'text',
          required: false,
          max: 10,
        },
        {
          name: 'weekday',
          type: 'number',
          required: false,
          min: 0,
          max: 6,
        },
        {
          name: 'color',
          type: 'text',
          required: true,
          max: 7,
          pattern: '^#[0-9A-Fa-f]{6}$',
        },
      ],
      indexes: [
        'CREATE UNIQUE INDEX idx_zones_weekday ON zones (weekday)',
      ],
    })
    
    $app.save(zones)
    console.log('Created zones collection')
  }
  
  // Create tasks collection if it doesn't exist
  if (!existingNames.includes('tasks')) {
    const zonesCollection = $app.findCollectionByNameOrId('zones')
    
    const tasks = new Collection({
      name: 'tasks',
      type: 'base',
      listRule: '',
      viewRule: '',
      createRule: '',
      updateRule: '',
      deleteRule: '',
      fields: [
        {
          name: 'name',
          type: 'text',
          required: true,
          max: 200,
        },
        {
          name: 'emoji',
          type: 'text',
          required: false,
          max: 10,
        },
        {
          name: 'zone',
          type: 'relation',
          required: false,
          collectionId: zonesCollection.id,
          maxSelect: 1,
          cascadeDelete: false,
        },
        {
          name: 'is_daily',
          type: 'bool',
          required: false,
        },
        {
          name: 'completed',
          type: 'bool',
          required: false,
        },
        {
          name: 'completed_at',
          type: 'date',
          required: false,
        },
        {
          name: 'sort_order',
          type: 'number',
          required: false,
          min: 0,
        },
      ],
      indexes: [],
    })
    
    $app.save(tasks)
    console.log('Created tasks collection')
  }
  
  // Create settings collection if it doesn't exist
  if (!existingNames.includes('settings')) {
    const settings = new Collection({
      name: 'settings',
      type: 'base',
      listRule: '',
      viewRule: '',
      createRule: '',
      updateRule: '',
      deleteRule: '',
      fields: [
        {
          name: 'key',
          type: 'text',
          required: true,
          max: 100,
        },
        {
          name: 'value',
          type: 'json',
          required: false,
        },
      ],
      indexes: [
        'CREATE UNIQUE INDEX idx_settings_key ON settings (key)',
      ],
    })
    
    $app.save(settings)
    console.log('Created settings collection')
  }
  
  console.log('Schema initialization complete')
})
