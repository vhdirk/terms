project(
  'terms',
  'rust',
  version: '0.1.0',
  meson_version: '>= 0.59',
  # license: 'MIT',
)

i18n = import('i18n')
gnome = import('gnome')

application_name = 'Terms'
base_id = 'com.github.vhdirk.' + application_name
application_id = base_id
full_version = '0.1.0'

# Check dependencies
glib_req_version = '2.72'
gtk_req_version = '4.10.0'
gir_req_version = '1.68'
libadwaita_req_version = '1.4.0'
libpanel_req_version = '1.4'

glib_req = '>= @0@'.format(glib_req_version)
gtk_req = '>= @0@'.format(gtk_req_version)
gir_req = '>= @0@'.format(gir_req_version)
libadwaita_req = '>= @0@'.format(libadwaita_req_version)
libpanel_req = '>= @0@'.format(libpanel_req_version)

dependency('glib-2.0', version: glib_req)
dependency('gio-2.0', version: glib_req) # always same version as glib
dependency('gtk4', version: gtk_req)
dependency('libadwaita-1', version: libadwaita_req)
dependency('libpanel-1', version: libpanel_req)

glib_compile_resources = find_program('glib-compile-resources', required: true)
glib_compile_schemas = find_program('glib-compile-schemas', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)
appstream_util = find_program('appstream-util', required: false)
cargo = find_program('cargo', required: true)
cargo_version = run_command(cargo, '--version', check: true).stdout().strip()
message(cargo_version)
rustc_version = run_command('rustc', '--version', check: true).stdout().strip()
message(rustc_version)

version = meson.project_version()

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')
datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'
podir = meson.project_source_root() / 'po'
gettext_package = meson.project_name()

if get_option('profile') == 'development'
  profile = 'Devel'
  application_id += '.Devel'
elif get_option('profile') == 'hack'
  profile = 'Devel'
  application_id += '.Hack'
elif get_option('profile') == 'beta'
  profile = 'Beta'
else
  profile = 'Stable'
endif

if profile == 'Devel'
  vcs_tag = run_command('git', 'rev-parse', '--short', 'HEAD', check: false).stdout().strip()
  if vcs_tag == ''
    version_suffix = '-devel'
  else
    version_suffix = '-@0@'.format(vcs_tag)
  endif
  application_id = '@0@.@1@'.format(base_id, profile)
else
  version_suffix = ''
  application_id = base_id
  release_date = run_command('git', 'show', '-s', '--format=%cI').stdout().strip()
endif
full_version += version_suffix


if profile == 'Devel'
  # Setup pre-commit hook for ensuring coding style is always consistent
  message('Setting up git pre-commit hook..')
  run_command('cp', '-f', 'hooks/pre-commit.hook', '.git/hooks/pre-commit', check: false)
endif

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + version,
  meson.project_source_root()
)

subdir('data')
subdir('po')
subdir('src')

summary({
    'prefix': get_option('prefix'),
    'libdir': get_option('libdir'),
    'datadir': get_option('datadir'),
    'bindir': get_option('bindir'),
  },
  section: 'Directories',
)

summary({
    'Profile': get_option('profile'),
  },
  section: 'Build options',
)

gnome.post_install(
  gtk_update_icon_cache: true,
  glib_compile_schemas: true,
  update_desktop_database: true,
)
