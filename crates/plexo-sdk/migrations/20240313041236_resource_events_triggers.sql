CREATE OR REPLACE FUNCTION notify_table_update() RETURNS TRIGGER AS $$
    DECLARE
    row RECORD;
    output TEXT;
    name TEXT;
    
    BEGIN

    IF (TG_OP = 'DELETE') THEN
      row = OLD;
    ELSE
      row = NEW;
    END IF;

    name = TG_TABLE_NAME || '_table_update';
    output = TG_TABLE_NAME || ' ' || TG_OP || ' ' || row.id;

    PERFORM pg_notify(name, output);
    
    RETURN NULL;

    END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trigger_tasks_table_update
  AFTER INSERT OR UPDATE OR DELETE
  ON tasks
  FOR EACH ROW
  EXECUTE PROCEDURE notify_table_update();


CREATE OR REPLACE TRIGGER trigger_projects_table_update
  AFTER INSERT OR UPDATE OR DELETE
  ON projects
  FOR EACH ROW
  EXECUTE PROCEDURE notify_table_update();

CREATE OR REPLACE TRIGGER trigger_members_table_update
  AFTER INSERT OR UPDATE OR DELETE
  ON members
  FOR EACH ROW
  EXECUTE PROCEDURE notify_table_update();

CREATE OR REPLACE TRIGGER trigger_teams_table_update
  AFTER INSERT OR UPDATE OR DELETE
  ON teams
  FOR EACH ROW
  EXECUTE PROCEDURE notify_table_update();

CREATE OR REPLACE TRIGGER trigger_labels_table_update
  AFTER INSERT OR UPDATE OR DELETE
  ON labels
  FOR EACH ROW
  EXECUTE PROCEDURE notify_table_update();

CREATE OR REPLACE TRIGGER trigger_assets_table_update
  AFTER INSERT OR UPDATE OR DELETE
  ON assets
  FOR EACH ROW
  EXECUTE PROCEDURE notify_table_update();

CREATE OR REPLACE TRIGGER trigger_changes_table_update
  AFTER INSERT OR UPDATE OR DELETE
  ON changes
  FOR EACH ROW
  EXECUTE PROCEDURE notify_table_update();