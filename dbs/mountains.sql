CREATE TABLE mountains (
    id integer primary key autoincrement,
    name text,
    height integer,
    country text,
    range text
);

CREATE INDEX idx_mountains_country ON mountains(country);

INSERT INTO mountains (name, height, country, range)
VALUES
    ('Teide',               3715,   'Spain',        null),
    ('Zugspitze',           2962,   'Germany',      'Eastern Alps'),
    ('Mont Blanc',          4810,   'France',       'Graian Alps'),
    ('Aneto',               3404,   'Spain',	    'Pyrenees'),
    ('Dufourspitze',        4634,   'Switzerland',  'Pennine Alps'),
    ('Dom',                 4545,   'Switzerland',  'Pennine Alps'),
    ('Gran Paradiso',       4061,   'Italy',        'Graian Alps'),
    ('Wildspitze',          3770,   'Austria',      'Ötztal Alps'),
    ('Mulhacén',            3482,   'Spain',	    'Sierra Nevada'),
    ('Weisshorn',           4506,   'Switzerland',  'Pennine Alps'),
    ('Montanha do Pico',    2351,   'Portugal',     null),
    ('Großglockner',        3798,   'Austria',      'Hohe Tauern'),
    ('Triglav',	            2864,   'Slovenia',     'Julian Alps'),
    ('Barre des Écrins',    4102,   'France',       'Dauphiné Alps'),
    ('Mont Blanc',          4805,   'Italy',	    'Graian Alps');
