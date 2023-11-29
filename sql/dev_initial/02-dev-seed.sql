INSERT INTO "user" (user_name)
VALUES ('Pupu-The-Tester');

INSERT INTO project (title, content)
VALUES ('test_project', 'test_content');

INSERT INTO "tags" (project_id, name)
VALUES (1000, 'kafka');
INSERT INTO "tags" (project_id, name)
VALUES (1000, 'apache');

INSERT INTO stages (name, description)
VALUES ('Formulation',
        'At this stage, we formulate all aspects of the design and features of the service prototype. All the necessary dependencies and technologies that will be used during the operation of the service.')