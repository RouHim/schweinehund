const PocketBase = require('pocketbase/cjs')

async function seed() {
  const pb = new PocketBase('http://127.0.0.1:8090')
  
  try {
    const zones = await pb.collection('zones').getFullList()
    if (zones.length > 0) {
      console.log('Zones already seeded')
      return
    }
  } catch (e) {
    console.log('Zones collection not ready yet')
    return
  }
  
  const zoneData = [
    { name: 'Sonntag - Frei', emoji: '🛋️', weekday: 0, color: '#9333EA' },
    { name: 'Montag (HO) - EG', emoji: '🏠', weekday: 1, color: '#3B82F6' },
    { name: 'Dienstag (HO) - KG', emoji: '🧺', weekday: 2, color: '#10B981' },
    { name: 'Mittwoch (HO) - OG', emoji: '🛁', weekday: 3, color: '#F59E0B' },
    { name: 'Donnerstag (Büro) - Leicht', emoji: '📋', weekday: 4, color: '#EF4444' },
    { name: 'Freitag (Büro) - Reset', emoji: '🔄', weekday: 5, color: '#8B5CF6' },
    { name: 'Samstag - Frei', emoji: '🎉', weekday: 6, color: '#EC4899' },
  ]
  
  for (const data of zoneData) {
    await pb.collection('zones').create(data)
  }
  console.log('Seeded zones')
  
  const zones_loaded = await pb.collection('zones').getFullList({ sort: 'weekday' })
  const mondayZone = zones_loaded.find(z => z.weekday === 1)
  const tuesdayZone = zones_loaded.find(z => z.weekday === 2)
  const wednesdayZone = zones_loaded.find(z => z.weekday === 3)
  const thursdayZone = zones_loaded.find(z => z.weekday === 4)
  const fridayZone = zones_loaded.find(z => z.weekday === 5)
  
  const taskData = [
    { name: 'Spülmaschine anstellen', emoji: '🍽️', is_daily: true, sort_order: 1 },
    { name: 'Spülmaschine ausräumen', emoji: '🍴', is_daily: true, sort_order: 2 },
    { name: 'Küche grob aufräumen', emoji: '🧹', is_daily: true, sort_order: 3 },
    { name: '1 Wäschegang (waschen + aufhängen)', emoji: '👕', is_daily: true, sort_order: 4 },
    { name: '5 Min Aufräumen (gemeinsam)', emoji: '⏱️', is_daily: true, sort_order: 5 },
    { name: 'Alle Oberflächen frei', emoji: '✨', is_daily: true, sort_order: 6 },
    
    { name: 'Küche: Herd, Spüle, Arbeitsflächen', emoji: '🏠', zone: mondayZone.id, is_daily: false, sort_order: 10 },
    { name: 'Esstisch abwischen', emoji: '🍽️', zone: mondayZone.id, is_daily: false, sort_order: 11 },
    { name: 'WC putzen', emoji: '🚽', zone: mondayZone.id, is_daily: false, sort_order: 12 },
    { name: 'EG durchsaugen & wischen', emoji: '🧽', zone: mondayZone.id, is_daily: false, sort_order: 13 },
    
    { name: 'Waschmaschine reinigen', emoji: '🧺', zone: tuesdayZone.id, is_daily: false, sort_order: 20 },
    { name: 'Müll rausbringen (alle Eimer)', emoji: '🗑️', zone: tuesdayZone.id, is_daily: false, sort_order: 21 },
    { name: 'KG aufräumen & durchsaugen', emoji: '🏚️', zone: tuesdayZone.id, is_daily: false, sort_order: 22 },
    
    { name: 'Badezimmer putzen (Dusche, Waschbecken)', emoji: '🛁', zone: wednesdayZone.id, is_daily: false, sort_order: 30 },
    { name: 'Betten neu beziehen', emoji: '🛏️', zone: wednesdayZone.id, is_daily: false, sort_order: 31 },
    { name: 'Wäsche zusammenlegen', emoji: '👔', zone: wednesdayZone.id, is_daily: false, sort_order: 32 },
    { name: 'OG durchsaugen', emoji: '🧹', zone: wednesdayZone.id, is_daily: false, sort_order: 33 },
    
    { name: 'Staub wischen (sichtbare Flächen)', emoji: '🪶', zone: thursdayZone.id, is_daily: false, sort_order: 40 },
    { name: 'Papierkram sortieren', emoji: '📄', zone: thursdayZone.id, is_daily: false, sort_order: 41 },
    
    { name: 'Restmüll + Biomüll raus', emoji: '♻️', zone: fridayZone.id, is_daily: false, sort_order: 50 },
    { name: 'Wäsche von Woche zusammenlegen', emoji: '👕', zone: fridayZone.id, is_daily: false, sort_order: 51 },
    { name: 'Gäste-WC Check', emoji: '🚿', zone: fridayZone.id, is_daily: false, sort_order: 52 },
    
    { name: 'Bad gründliche Reinigung (Fugen, Armaturen)', emoji: '🧼', is_daily: false, sort_order: 100 },
    { name: 'Kühlschrank ausputzen', emoji: '❄️', is_daily: false, sort_order: 101 },
    { name: 'Fenster putzen (alle)', emoji: '🪟', is_daily: false, sort_order: 102 },
    { name: 'Schrank aussortieren (Kleidung, Spielzeug)', emoji: '📦', is_daily: false, sort_order: 103 },
  ]
  
  for (const data of taskData) {
    await pb.collection('tasks').create(data)
  }
  console.log('Seeded tasks')
}

seed().catch(console.error)
