root_path=$(dirname $(realpath $0))
root_name=$(basename $root_path)

export http_proxy=${http_proxy:-'http://127.0.0.1:1080'}
export https_proxy=${https_proxy:-'http://127.0.0.1:1080'}

worksapce='rustiny'
members=(
    'number'
    'fixed_point'
    'linear_algebra'
    'computer_graphic'
    'xfile'
    'renderer'
)

function color_ps3() {
    local prompt=${1:-'Select '}

    echo $'\e[32m'$prompt$'\e[m'
}

function color_msg() {
    local color=${1:?'(r)ed or (g)reen (b)lue (y)ellow (p)urple (c)yan'}

    if (( 2 > $# )); then
        return
    fi

    if [[ 'r' == $color ]]; then
        echo -e '\e[31m'${@:2}'\e[0m' # red
    elif [[ 'g' == $color ]]; then
        echo -e '\e[32m'${@:2}'\e[0m' # green
    elif [[ 'b' == $color ]]; then
        echo -e '\e[34m'${@:2}'\e[0m' # blue
    elif [[ 'y' == $color ]]; then
        echo -e '\e[33m'${@:2}'\e[0m' # yellow
    elif [[ 'p' == $color ]]; then
        echo -e '\e[35m'${@:2}'\e[0m' # purple
    elif [[ 'c' == $color ]]; then
        echo -e '\e[36m'${@:2}'\e[0m' # cyan
    else
        echo -e '\e[37m'${@:2}'\e[0m' # white
    fi
}

function get_unique_name() {
    local source_file=${1:?''}

    local source_file_name=`basename $source_file`

    local id_name=${source_file_name%.*}

    if [[ 'main' == $id_name || 'lib' == $id_name || 'bin' == $id_name ]]; then
        local dir=`dirname $source_file`
        while true; do
            id_name=`basename $dir`
            if [[ 'src' != $id_name ]]; then
                break
            fi
            dir=`dirname $dir`
        done
    fi

    echo $id_name
}

function exec_cargo() {
    local need_input=${1:-'y'}
    local cargo_args=(${@:2})

    if [[ 'y' == $need_input ]]; then
        color_msg y "preparing execute: cargo ${cargo_args[*]}"

        read -p "input cargo ext args: " ext_cargo_args

        for ext_arg in ${ext_cargo_args[@]}; do
            cargo_args+=($ext_arg)
        done
    else
        cargo_args=(${@:1})
    fi

    color_msg g "executing: cargo ${cargo_args[*]}"

    cargo ${cargo_args[@]}

    return $?
}

function select_member() {
    local workspace=${1:-''}

    if [[ -n $workspace ]]; then
        PS3=`color_ps3 "select cargo package (1-$((${#members[@]}+1))): "`
        select label in $workspace ${members[@]}; do
            label=${label:-$workspace}
            echo $label
            break
        done
    else
        PS3=`color_ps3 "select cargo package (1-${#members[@]}): "`
        select label in ${members[@]}; do
            local index=$(($REPLY-1))
            if (( 0 > $index || $index >= ${#members[@]} )); then
                index=0
            fi
            echo ${members[$index]}
            break
        done
    fi
}

function count_main() {
    local root_path_to_build=${1:?''}

    local builds=()

    local main_path=$root_path_to_build/src
    for build in `find $main_path -maxdepth 1 -type f | grep -P 'main.rs$'`; do
        builds+=($build)
    done

    # local ext_paths=(
    #     $root_path_to_build/src/bin
    # )
    # for ext_path in ${ext_paths[@]}; do
    #     if [[ -d $ext_path ]]; then
    #         for build in `find $ext_path -maxdepth 1 -type f | grep -P '[^\s]+?\.rs$'`; do
    #             builds+=($build)
    #         done
    #         for build in `find $ext_path -type f | grep -P 'main.rs$'`; do
    #             builds+=($build)
    #         done
    #     fi
    # done

    echo ${#builds[@]}
}

function count_bins() {
    local root_path_to_build=${1:?''}

    local builds=()

    # local main_path=$root_path_to_build/src
    # for build in `find $main_path -maxdepth 1 -type f | grep -P 'main.rs$'`; do
    #     builds+=($build)
    # done

    local ext_paths=(
        $root_path_to_build/src/bin
    )
    for ext_path in ${ext_paths[@]}; do
        if [[ -d $ext_path ]]; then
            for build in `find $ext_path -maxdepth 1 -type f | grep -P '[^\s]+?\.rs$'`; do
                builds+=($build)
            done
            for build in `find $ext_path -type f | grep -P 'main.rs$'`; do
                builds+=($build)
            done
        fi
    done

    echo ${#builds[@]}
}

function select_bin() {
    local root_path_to_build=${1:?''}

    local builds=()

    # local main_path=$root_path_to_build/src
    # for build in `find $main_path -maxdepth 1 -type f | grep -P 'main.rs$'`; do
    #     builds+=($build)
    # done

    local ext_paths=(
        $root_path_to_build/src/bin
    )
    for ext_path in ${ext_paths[@]}; do
        if [[ -d $ext_path ]]; then
            for build in `find $ext_path -maxdepth 1 -type f | grep -P '[^\s]+?\.rs$'`; do
                builds+=($build)
            done
            for build in `find $ext_path -type f | grep -P 'main.rs$'`; do
                builds+=($build)
            done
        fi
    done

    PS3=`color_ps3 "select bin (1-${#builds[@]}): "`
    select label in ${builds[@]}; do
        local index=$(($REPLY-1))
        if (( 0 > $index || $index >= ${#builds[@]} )); then
            index=0
        fi
        echo ${builds[$index]}
        break
    done
}

function count_examples() {
    local root_path_to_build=${1:?''}

    local builds=()

    # local main_path=$root_path_to_build/src
    # for build in `find $main_path -maxdepth 1 -type f | grep -P 'main.rs$'`; do
    #     builds+=($build)
    # done

    local ext_paths=(
        $root_path_to_build/examples
    )
    for ext_path in ${ext_paths[@]}; do
        if [[ -d $ext_path ]]; then
            for build in `find $ext_path -maxdepth 1 -type f | grep -P '[^\s]+?\.rs$'`; do
                builds+=($build)
            done
            for build in `find $ext_path -type f | grep -P 'main.rs$'`; do
                builds+=($build)
            done
        fi
    done

    echo ${#builds[@]}
}

function select_example() {
    local root_path_to_build=${1:?''}

    local builds=()

    # local main_path=$root_path_to_build/src
    # for build in `find $main_path -maxdepth 1 -type f | grep -P 'main.rs$'`; do
    #     builds+=($build)
    # done

    local ext_paths=(
        $root_path_to_build/examples
    )
    for ext_path in ${ext_paths[@]}; do
        if [[ -d $ext_path ]]; then
            for build in `find $ext_path -maxdepth 1 -type f | grep -P '[^\s]+?\.rs$'`; do
                builds+=($build)
            done
            for build in `find $ext_path -type f | grep -P 'main.rs$'`; do
                builds+=($build)
            done
        fi
    done

    PS3=`color_ps3 "select example (1-${#builds[@]}): "`
    select label in ${builds[@]}; do
        local index=$(($REPLY-1))
        if (( 0 > $index || $index >= ${#builds[@]} )); then
            index=0
        fi
        echo ${builds[$index]}
        break
    done
}

function cargo_build_or_run() {
    local act=${1:?''}

    local member=`select_member`

    local labels=(
        "main(`count_main $root_path/$member`)"
        "bin(`count_bins $root_path/$member`)"
        "example(`count_examples $root_path/$member`)"
    )

    local groups=(
        'main'
        'bin'
        'example'
    )

    local group=

    PS3=`color_ps3 "select group (1-${#labels[@]}): "`
    select label in ${labels[@]}; do
        local index=$(($REPLY-1))
        if (( 0 > $index || $index >= ${#labels[@]} )); then
            index=0
        fi
        group=${groups[$index]}
        break
    done

    if [[ 'main' = $group ]]; then
        if (( 0 < `count_main $root_path/$member` )); then
            exec_cargo $act -p ${worksapce}_$member
        else
            color_msg y 'no main'
        fi
    elif [[ 'bin' == $group ]]; then
        local bin=`select_bin $root_path/$member`
        if [[ -n $bin ]]; then
            exec_cargo $act -p ${worksapce}_$member --bin `get_unique_name $bin`
        else
            color_msg y 'no bins'
        fi
    elif [[ 'example' == $group ]]; then
        local example=`select_example $root_path/$member`
        if [[ -n $example ]]; then
            exec_cargo $act -p ${worksapce}_$member --example `get_unique_name $example`
        else
            color_msg y 'no examples'
        fi
    fi
}

function cargo_build() {
    cargo_build_or_run 'build'
}

function cargo_run() {
    cargo_build_or_run 'run'
}

function cargo_doc() {
    local member=`select_member workspace`

    if [[ 'workspace' == $member ]]; then
        exec_cargo doc --workspace
    else
        exec_cargo doc -p ${worksapce}_$member
    fi
}

function cargo_publish() {
    exec_cargo y publish -p ${worksapce}_`select_member`
}

function select_cargo_command() {
    local commands=(
        'run'
        'build'
        'doc'
        'publish'
    )

    PS3=`color_ps3 "select cargo command (1-${#commands[@]}): "`
    select label in ${commands[@]}; do
        local index=$(($REPLY-1))
        if (( 0 > $index || $index >= ${#commands[@]} )); then
            index=0
        fi
        cargo_${commands[$index]}
        break
    done
}

function main() {
    if [[ -n $http_proxy || -n $https_proxy ]]; then
        color_msg y "using proxy: $http_proxy, $https_proxy"
    fi

    select_cargo_command
}
main