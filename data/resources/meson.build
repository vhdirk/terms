# Resources
fs = import('fs')
subdir('icons')

# stylesheet_deps = []
# sassc = find_program('sassc')

# if sassc.found()
#   sassc_opts = [ '-a', '-M', '-t', 'compact' ]

#    scss_files = [
#       'style',
#       'style-dark',
#     ]

#     foreach scss: scss_files
#       stylesheet_deps += custom_target('@0@.scss'.format(scss),
#         input: '@0@.scss'.format(scss),
#         output: '@0@.css'.format(scss),
#         command: [
#           sassc, sassc_opts, '@INPUT@', '@OUTPUT@',
#         ],
#       )
#     endforeach
# endif


resources = gnome.compile_resources(
  'resources',
  'resources.gresource.xml',
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
)
