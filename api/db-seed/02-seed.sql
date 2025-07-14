-- Seed data for categories
INSERT INTO categories (name) VALUES
  ('Main Dish'),
  ('Break Fast'),
  ('Dessert'),
  ('Breakfast Food'),
  ('Snack & Spice'),
  ('Dairy & Milk'),
  ('Juice & Drinks'),
  
  -- Seed data for products
  INSERT INTO products (name, description, price, isBestSeller, isDealOfTheDay, discount, category_id) VALUES
    -- Main Dish (1)
    ('Grilled Chicken Breast', 'Tender grilled chicken seasoned with herbs.', 75.00, TRUE, FALSE, 10.0, 1),
    ('Beef Lasagna', 'Classic Italian lasagna layered with beef and cheese.', 85.50, FALSE, TRUE, 15.0, 1),
    ('Vegetable Stir Fry', 'Fresh veggies sautéed in soy-ginger sauce.', 65.00, FALSE, FALSE, 0.0, 1),
  
    -- Break Fast (2)
    ('Simit & Cheese Plate', 'Traditional Turkish simit served with white cheese.', 30.00, TRUE, FALSE, 0.0, 2),
    ('Egg & Sujuk Pan', 'Pan-fried eggs with spicy Turkish sausage.', 42.00, FALSE, TRUE, 10.0, 2),
  
    -- Dessert (3)
    ('Baklava', 'Rich and sweet pastry made of layers of filo and nuts.', 25.00, TRUE, TRUE, 20.0, 3),
    ('Chocolate Mousse', 'Creamy chocolate dessert topped with whipped cream.', 18.00, FALSE, FALSE, 0.0, 3),
  
    -- Breakfast Food (4)
    ('Menemen', 'Turkish-style scrambled eggs with tomatoes and peppers.', 35.00, TRUE, FALSE, 5.0, 4),
    ('Peynirli Börek', 'Flaky pastry filled with feta cheese.', 28.00, FALSE, FALSE, 0.0, 4),
  
    -- Snack & Spice (5)
    ('Chili Roasted Almonds', 'Crunchy almonds with a spicy chili kick.', 19.00, TRUE, FALSE, 0.0, 5),
    ('Paprika Potato Chips', 'Hand-cut chips tossed in smoky paprika.', 15.00, FALSE, TRUE, 10.0, 5),
  
    -- Dairy & Milk (6)
    ('Whole Milk (1L)', 'Fresh full-fat milk.', 14.50, TRUE, FALSE, 0.0, 6),
    ('Aged Kaşar Cheese', 'Rich and tangy Turkish aged cheese.', 38.00, FALSE, TRUE, 20.0, 6),
  
    -- Juice & Drinks (7)
    ('Fresh Orange Juice (250ml)', 'Cold-pressed, freshly squeezed orange juice.', 22.00, TRUE, FALSE, 0.0, 7),
    ('Ayran (Salted Yogurt Drink)', 'Refreshing yogurt-based Turkish drink.', 12.00, FALSE, TRUE, 5.0, 7);