# Description: Converts all .yaml files in the _data directory to .json files in the api directory
require 'json'
require 'safe_yaml'

module Jekyll
  class YamlToJsonConverter < Generator
    safe true
    priority :low

    def generate(site)
      puts "Generating JSON files from YAML data..."
      # Create the `api` directory if it doesn't exist
      api_dir = File.join(site.source, "api")
      Dir.mkdir(api_dir) unless Dir.exist?(api_dir)

      # Iterate through all .yaml files in the `_data` directory
      data_dir = File.join(site.source, "_data")

      Dir.glob(File.join(data_dir, "*.yml")) do |yaml_file|
        puts "    #{yaml_file}"
        # Read the .yaml file and parse its contents
        yaml_data = SafeYAML.load_file(yaml_file)

        # Convert the data to JSON and create a JSON file corresponding to the .yaml file
        json_file_name = File.basename(yaml_file, ".yml") + ".json"
        puts "  #{json_file_name}"
        json_file_path = File.join(api_dir, json_file_name)
        File.open(json_file_path, "w") do |json_file|
          json_file.write(JSON.pretty_generate(yaml_data))
        end
      end
    end
  end
end
