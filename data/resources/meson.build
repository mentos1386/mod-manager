# Resources
blueprints = custom_target('blueprints',
  input: files(
    'ui/windows/main/main.blp',
    'ui/windows/main/pages/welcome.blp',
    'ui/windows/main/pages/games_and_mods.blp',
    'ui/windows/add_new_game/add_new_game.blp',
    'ui/components/mods_list.blp',
  ),
  output: '.',
  command: [find_program('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)
resources = gnome.compile_resources(
  'resources',
  'resources.gresource.xml',
  gresource_bundle: true,
  dependencies: blueprints,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
)
