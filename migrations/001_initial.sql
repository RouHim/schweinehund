-- Initial schema for Schweinehund cleaning task manager

CREATE TABLE daily_tasks (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    zone TEXT,
    day_of_week INTEGER,
    completed BOOLEAN DEFAULT 0,
    completed_at INTEGER
);

CREATE TABLE deep_cleaning_tasks (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    queue_position INTEGER NOT NULL,
    completed_at INTEGER
);

CREATE TABLE app_state (
    key TEXT PRIMARY KEY,
    value TEXT
);

-- Daily Mini-Routine (every day: day_of_week = -1)
INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at) VALUES
('Spuelmaschine an/aus', 'Spülmaschine starten oder ausräumen', NULL, -1, 0, NULL),
('Kueche grob aufraeumen', 'Küche aufräumen und Arbeitsplatz frei machen', 'EG - Wohnbereich/Kueche/WC', -1, 0, NULL),
('1 Waeschegang oder Waesche falten', 'Wäsche waschen oder bereits gewaschene Wäsche falten', NULL, -1, 0, NULL),
('5 Min gemeinsames Aufraeumen', 'Kurzes Aufräumen mit der Familie', NULL, -1, 0, NULL),
('Oberflaechen frei machen', 'Oberflächen von Gegenständen befreien', NULL, -1, 0, NULL);

-- Monday (1): EG - Wohnbereich/Kueche/WC
INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at) VALUES
('Kueche: Arbeitsflaechen, Herd, Spuele', 'Küche reinigen: Arbeitsplatz, Herd und Spüle', 'EG - Wohnbereich/Kueche/WC', 1, 0, NULL),
('Esstisch & Couchtisch abwischen', 'Tische abwischen und säubern', 'EG - Wohnbereich/Kueche/WC', 1, 0, NULL),
('WC kurz reinigen', 'Toilette reinigen und säubern', 'EG - Wohnbereich/Kueche/WC', 1, 0, NULL),
('Boden: nur WISCHEN', 'Boden wischen (nicht saugen)', 'EG - Wohnbereich/Kueche/WC', 1, 0, NULL);

-- Tuesday (2): KG - Keller/Waschen
INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at) VALUES
('Waschmaschine & Trockner', 'Waschmaschine und Trockner aufräumen/warten', 'KG - Keller/Waschen', 2, 0, NULL),
('Leere Kartons / Muell raus', 'Leere Kartons und Müll hinausbringen', 'KG - Keller/Waschen', 2, 0, NULL),
('1 Ecke / 1 Regal ordnen', 'Eine Ecke oder ein Regal organisieren', 'KG - Keller/Waschen', 2, 0, NULL);

-- Wednesday (3): 1.OG - Schlaf/Kind/Bad
INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at) VALUES
('Bad: WC, Waschbecken, Spiegel', 'Badezimmer reinigen: WC, Waschbecken und Spiegel', '1.OG - Schlaf/Kind/Bad', 3, 0, NULL),
('Betten richten', 'Betten machen und Bettzeug aufschütteln', '1.OG - Schlaf/Kind/Bad', 3, 0, NULL),
('Waesche einsammeln', 'Schmutzige Wäsche einsammeln', '1.OG - Schlaf/Kind/Bad', 3, 0, NULL),
('Saugen', 'Saugen und Böden reinigen', '1.OG - Schlaf/Kind/Bad', 3, 0, NULL);

-- Thursday (4): Light - Büro day
INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at) VALUES
('Staub wischen (1-2 Raeume)', 'Staub abwischen in 1-2 Räumen', 'Buero', 4, 0, NULL),
('Papierkram einsammeln', 'Papiere organisieren und sortieren', 'Buero', 4, 0, NULL),
('Dinge zuruecklegen', 'Gegenstände an ihren Platz zurückbringen', NULL, 4, 0, NULL);

-- Friday (5): Wochen-Reset
INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at) VALUES
('Muell raus', 'Müll hinausbringen für die Woche', NULL, 5, 0, NULL),
('Waesche falten', 'Ganze Woche Wäsche falten', NULL, 5, 0, NULL),
('Oberflaechen frei', 'Alle Oberflächen befreien', NULL, 5, 0, NULL),
('Bad-Check (Handtuecher, WC)', 'Badezimmer kontrollieren: Handtücher, WC säubern', '1.OG - Schlaf/Kind/Bad', 5, 0, NULL);

-- Deep cleaning tasks (rotation queue)
INSERT INTO deep_cleaning_tasks (name, description, queue_position, completed_at) VALUES
('Bad gruendlich', 'Gründliche Badreinigung - alle Ecken und Fugen', 1, NULL),
('Kuehlschrank', 'Kühlschrank ausräumen und gründlich reinigen', 2, NULL),
('Fenster putzen', 'Alle Fenster putzen - innen und außen', 3, NULL),
('Schrank/Spielzeug aussortieren', 'Schrank oder Spielzeug sortieren und entrümpeln', 4, NULL);

-- App state initialization
INSERT INTO app_state (key, value) VALUES
('last_reset_at', '0'),
('notification_enabled', 'true'),
('notification_time', '09:00');
